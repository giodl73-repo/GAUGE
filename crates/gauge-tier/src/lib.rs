use gauge_corpus::{CorpusEntry, EvidenceLabel};
use gauge_network::{Network, NetworkError};
use gauge_score::{Dimension, Score, ScoreError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Tier {
    T1,
    T2,
    T3,
    T4,
}

impl Tier {
    pub const ALL: [Self; 4] = [Self::T1, Self::T2, Self::T3, Self::T4];

    pub fn key(self) -> &'static str {
        match self {
            Self::T1 => "T1",
            Self::T2 => "T2",
            Self::T3 => "T3",
            Self::T4 => "T4",
        }
    }

    pub fn parse(value: &str) -> Result<Self, TierError> {
        match value.trim().to_ascii_uppercase().as_str() {
            "T1" => Ok(Self::T1),
            "T2" => Ok(Self::T2),
            "T3" => Ok(Self::T3),
            "T4" => Ok(Self::T4),
            other => Err(TierError::UnknownTier(other.to_string())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sla {
    pub tier: Tier,
    pub trip_time: String,
    pub frequency: String,
    pub reliability: String,
    pub access: String,
    pub evidence_label: EvidenceLabel,
    pub rationale: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gap {
    pub entry_id: String,
    pub tier: Tier,
    pub dimension: Dimension,
    pub score: Score,
    pub reason: String,
    pub basis: String,
}

pub fn classify(entry: &CorpusEntry) -> Result<Tier, TierError> {
    let tier = entry.tier.as_deref().ok_or(TierError::MissingTier)?;
    Tier::parse(tier)
}

pub fn default_sla(tier: Tier) -> Sla {
    let (trip_time, frequency, reliability, access) = match tier {
        Tier::T1 => (
            "fastest scheduled time",
            "highest frequency",
            "highest reliability",
            "premier corridor access",
        ),
        Tier::T2 => (
            "fast intercity time",
            "frequent service",
            "high reliability",
            "intercity access",
        ),
        Tier::T3 => (
            "moderate scheduled time",
            "scheduled service",
            "moderate reliability",
            "regional access",
        ),
        Tier::T4 => (
            "basic scheduled time",
            "limited service",
            "baseline reliability",
            "feeder access",
        ),
    };
    Sla {
        tier,
        trip_time: trip_time.to_string(),
        frequency: frequency.to_string(),
        reliability: reliability.to_string(),
        access: access.to_string(),
        evidence_label: EvidenceLabel::Planned,
        rationale: "Provisional tier-SLA record pending calibrated operational evidence."
            .to_string(),
    }
}

pub fn conformance(entry: &CorpusEntry, network: &Network) -> Result<Score, TierError> {
    classify(entry)?;
    let required_min = required_trip_time_min(entry).ok_or(TierError::MissingTripTimeMin)?;
    let path: Vec<&str> = entry
        .termini
        .iter()
        .map(String::as_str)
        .filter(|terminus| !terminus.trim().is_empty())
        .collect();
    if path.len() < 2 {
        return Err(TierError::MissingTerminus);
    }
    let observed_min = network.trip_time_minutes(&path)?;
    let score = if observed_min <= 0.0 {
        10.0
    } else {
        (required_min / observed_min * 10.0).clamp(0.0, 10.0)
    };
    Score::new(score).map_err(TierError::Score)
}

pub fn tier_sla_gap(entry: &CorpusEntry) -> Result<Option<Gap>, TierError> {
    let tier = classify(entry)?;
    let Some(raw_score) = entry.scores.get(Dimension::Dim13.key()).copied() else {
        return Ok(None);
    };
    gap_from_score(entry, tier, Score::new(raw_score)?)
}

pub fn tier_sla_gap_with_network(
    entry: &CorpusEntry,
    network: &Network,
) -> Result<Option<Gap>, TierError> {
    let tier = classify(entry)?;
    gap_from_score(entry, tier, conformance(entry, network)?)
}

fn gap_from_score(entry: &CorpusEntry, tier: Tier, score: Score) -> Result<Option<Gap>, TierError> {
    if score.value() >= 10.0 {
        return Ok(None);
    }

    Ok(Some(Gap {
        entry_id: entry.id.clone().ok_or(TierError::MissingEntryIdForGap)?,
        tier,
        dimension: Dimension::Dim13,
        score,
        reason: "DIM-13 score below full tier-SLA conformance".to_string(),
        basis: "tier SLA evaluated on free-flow trip-time / dispatch basis".to_string(),
    }))
}

fn required_trip_time_min(entry: &CorpusEntry) -> Option<f64> {
    entry
        .quantities
        .iter()
        .find(|quantity| quantity.unit.eq_ignore_ascii_case("min"))
        .map(|quantity| quantity.value)
}

#[derive(Debug, Error, PartialEq)]
pub enum TierError {
    #[error("missing tier")]
    MissingTier,
    #[error("unknown tier: {0}")]
    UnknownTier(String),
    #[error("missing required trip-time (min) quantity")]
    MissingTripTimeMin,
    #[error("missing terminus for tier-SLA conformance")]
    MissingTerminus,
    #[error("missing entry id for tier-SLA gap")]
    MissingEntryIdForGap,
    #[error(transparent)]
    Network(#[from] NetworkError),
    #[error(transparent)]
    Score(#[from] ScoreError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use gauge_corpus::Quantity;
    use gauge_network::{Dispatch, Segment, Station};

    fn station(id: &str) -> Station {
        Station {
            id: id.to_string(),
            name: format!("{id} station"),
            state: "CA".to_string(),
        }
    }

    fn entry_with_tier(tier: &str) -> CorpusEntry {
        CorpusEntry {
            id: Some("corridor-1".to_string()),
            tier: Some(tier.to_string()),
            ..CorpusEntry::default()
        }
    }

    fn two_station_corridor() -> Network {
        let mut network = Network::new();
        network
            .add_station(station("a"))
            .expect("station a accepted");
        network
            .add_station(station("b"))
            .expect("station b accepted");
        network
            .add_segment(
                "a",
                "b",
                Segment {
                    id: "ab".to_string(),
                    miles: 120.0,
                    max_mph: 60.0,
                    dispatch: Dispatch::Dedicated,
                },
            )
            .expect("segment ab accepted");
        network
    }

    #[test]
    fn classify_reads_declared_tier() {
        let entry = entry_with_tier("T2");

        assert_eq!(classify(&entry), Ok(Tier::T2));
    }

    #[test]
    fn default_sla_records_provisional_label() {
        let sla = default_sla(Tier::T1);

        assert_eq!(sla.evidence_label, EvidenceLabel::Planned);
        assert!(sla.rationale.contains("Provisional"));
        assert_eq!(sla.frequency, "highest frequency");
    }

    #[test]
    fn conformance_scores_trip_time_against_required() {
        // 120 mi @ 60 mph = 120 min observed; required 120 min -> full conformance.
        let mut entry = entry_with_tier("T2");
        entry.termini = vec!["a".to_string(), "b".to_string()];
        entry.quantities.push(Quantity {
            value: 120.0,
            unit: "min".to_string(),
            label: Some(EvidenceLabel::Planned),
            source_id: None,
        });

        let score = conformance(&entry, &two_station_corridor()).expect("entry conforms");

        assert_eq!(score.value(), 10.0);
    }

    #[test]
    fn tier_sla_gap_reports_dim13_shortfall() {
        let mut entry = entry_with_tier("T3");
        entry.scores.insert("DIM-13".to_string(), 6.5);

        let gap = tier_sla_gap(&entry)
            .expect("gap evaluation succeeds")
            .expect("shortfall produces gap");

        assert_eq!(gap.entry_id, "corridor-1");
        assert_eq!(gap.tier, Tier::T3);
        assert_eq!(gap.dimension, Dimension::Dim13);
        assert_eq!(gap.score.value(), 6.5);
        assert!(gap.basis.contains("trip-time"));
    }

    #[test]
    fn full_dim13_score_has_no_gap() {
        let mut entry = entry_with_tier("T4");
        entry.scores.insert("DIM-13".to_string(), 10.0);

        assert_eq!(tier_sla_gap(&entry), Ok(None));
    }
}
