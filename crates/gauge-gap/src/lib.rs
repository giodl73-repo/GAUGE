use std::collections::BTreeSet;

use gauge_corpus::{CorpusEntry, EvidenceLabel};
use gauge_score::{Dimension, Rubric, Score, ScoreError};
use gauge_tier::{tier_sla_gap, TierError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

const UNDER_SERVED_THRESHOLD: f64 = 7.0;

/// Share of scored entries below threshold at or above which a dispersion gap is
/// reclassified from a concentrated *tail* to a *systemic* deficit. A bottom-
/// quartile trigger fires the same way for a 2-of-8 minority and an 8-of-12
/// majority; this constant lets the consumer tell "target the named minority"
/// apart from "the whole class is below the bar".
const SYSTEMIC_SHARE: f64 = 0.5;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GapRegion {
    pub id: String,
    pub dimension: Dimension,
    pub threshold: Score,
    pub observed: Option<Score>,
    pub entry_ids: Vec<String>,
    pub reason: String,
    pub source: GapSource,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GapSource {
    EmptyRegion,
    UnderServedRegion,
    TailRegion,
    SystemicRegion,
    TierSla,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NullResult {
    pub region_id: String,
    pub rubric_version: String,
    pub evidence_label: EvidenceLabel,
    pub rationale: String,
    pub inspected_entries: usize,
}

pub fn find_gaps(corpus: &[CorpusEntry], rubric: &Rubric) -> Result<Vec<GapRegion>, GapError> {
    let mut gaps = Vec::new();

    for dimension in Dimension::ALL {
        let threshold = threshold_for(rubric, dimension)?;
        let scored = scored_entries(corpus, dimension)?;
        if scored.is_empty() {
            gaps.push(GapRegion {
                id: format!("empty-{}", dimension.key()),
                dimension,
                threshold,
                observed: None,
                entry_ids: Vec::new(),
                reason: format!("no corpus entries scored for {}", dimension.key()),
                source: GapSource::EmptyRegion,
            });
            continue;
        }

        let lowest = scored
            .iter()
            .min_by(|left, right| left.1.value().total_cmp(&right.1.value()))
            .expect("scored entries are not empty");
        if lowest.1.value() < threshold.value() {
            gaps.push(GapRegion {
                id: format!("under-served-{}", dimension.key()),
                dimension,
                threshold,
                observed: Some(lowest.1),
                entry_ids: scored.iter().map(|(id, _)| id.clone()).collect(),
                reason: format!("lowest {} score is below threshold", dimension.key()),
                source: GapSource::UnderServedRegion,
            });
        }
        if let Some(gap) = tail_gap(dimension, threshold, &scored) {
            gaps.push(gap);
        }
    }

    gaps.extend(tier_sla_gap_regions(corpus)?);
    gaps.sort_by(|left, right| left.id.cmp(&right.id));
    gaps.dedup_by(|left, right| left.id == right.id);
    Ok(gaps)
}

pub fn null_result(
    region_id: impl Into<String>,
    corpus: &[CorpusEntry],
    rubric: &Rubric,
) -> Result<Option<NullResult>, GapError> {
    if !find_gaps(corpus, rubric)?.is_empty() {
        return Ok(None);
    }

    Ok(Some(NullResult {
        region_id: region_id.into(),
        rubric_version: rubric.version.clone(),
        evidence_label: EvidenceLabel::Implemented,
        rationale: "No gap manufactured: all rubric dimensions are scored at or above threshold and no tier-SLA shortfall is present. A car/air-dominant market is a valid null result.".to_string(),
        inspected_entries: corpus.len(),
    }))
}

fn threshold_for(rubric: &Rubric, dimension: Dimension) -> Result<Score, GapError> {
    let weight = rubric.weight(dimension).unwrap_or(1.0);
    let threshold = (UNDER_SERVED_THRESHOLD * weight.max(0.0)).clamp(0.0, 10.0);
    Ok(Score::new(threshold)?)
}

fn tail_gap(
    dimension: Dimension,
    threshold: Score,
    scored: &[(String, Score)],
) -> Option<GapRegion> {
    if scored.is_empty() {
        return None;
    }

    let mut sorted = scored.to_vec();
    sorted.sort_by(|left, right| left.1.value().total_cmp(&right.1.value()));
    let tail_len = sorted.len().div_ceil(4).max(1);
    let tail_sum: f64 = sorted.iter().take(tail_len).map(|(_, s)| s.value()).sum();
    let tail_mean = tail_sum / tail_len as f64;
    let below: Vec<String> = scored
        .iter()
        .filter(|(_, s)| s.value() < threshold.value())
        .map(|(id, _)| id.clone())
        .collect();

    if tail_mean >= threshold.value() || below.is_empty() {
        return None;
    }

    let below_count = below.len();
    let share = below_count as f64 / scored.len() as f64;
    let systemic = share >= SYSTEMIC_SHARE;
    let (id, source, kind) = if systemic {
        (
            format!("systemic-{}", dimension.key()),
            GapSource::SystemicRegion,
            "a systemic deficit (most of the class is below the bar)",
        )
    } else {
        (
            format!("tail-{}", dimension.key()),
            GapSource::TailRegion,
            "a concentrated tail (target the named minority)",
        )
    };

    Some(GapRegion {
        id,
        dimension,
        threshold,
        observed: Some(Score::new(tail_mean.clamp(0.0, 10.0)).ok()?),
        entry_ids: below,
        reason: format!(
            "bottom-quartile {} mean is below threshold; {} of {} scored entries ({:.0}%) fall below the bar, classified as {}",
            dimension.key(),
            below_count,
            scored.len(),
            share * 100.0,
            kind
        ),
        source,
    })
}

fn scored_entries(
    corpus: &[CorpusEntry],
    dimension: Dimension,
) -> Result<Vec<(String, Score)>, GapError> {
    let mut scored = Vec::new();
    for entry in corpus {
        let Some(raw_score) = entry.scores.get(dimension.key()).copied() else {
            continue;
        };
        let id = entry.id.clone().ok_or(GapError::MissingEntryId)?;
        scored.push((id, Score::new(raw_score)?));
    }
    Ok(scored)
}

fn tier_sla_gap_regions(corpus: &[CorpusEntry]) -> Result<Vec<GapRegion>, GapError> {
    let mut regions = Vec::new();
    let mut seen = BTreeSet::new();
    for entry in corpus {
        let Some(gap) = tier_sla_gap(entry)? else {
            continue;
        };
        if !seen.insert(gap.entry_id.clone()) {
            continue;
        }
        regions.push(GapRegion {
            id: format!("tier-sla-{}", gap.entry_id),
            dimension: gap.dimension,
            threshold: Score::new(10.0)?,
            observed: Some(gap.score),
            entry_ids: vec![gap.entry_id],
            reason: gap.reason,
            source: GapSource::TierSla,
        });
    }
    Ok(regions)
}

#[derive(Debug, Error, PartialEq)]
pub enum GapError {
    #[error("gap analysis requires scored entries to have stable ids")]
    MissingEntryId,
    #[error(transparent)]
    Score(#[from] ScoreError),
    #[error(transparent)]
    Tier(#[from] TierError),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fully_scored_entry(id: &str, score: f64) -> CorpusEntry {
        let mut entry = CorpusEntry {
            id: Some(id.to_string()),
            tier: Some("T1".to_string()),
            ..CorpusEntry::default()
        };
        for dimension in Dimension::ALL {
            entry.scores.insert(dimension.key().to_string(), score);
        }
        entry
    }

    #[test]
    fn find_gaps_reports_under_served_dimension() {
        let mut entry = fully_scored_entry("corridor-1", 9.0);
        entry.scores.insert("DIM-05".to_string(), 4.5);

        let gaps = find_gaps(&[entry], &Rubric::default_v0()).expect("gap analysis succeeds");

        assert!(gaps.iter().any(|gap| {
            gap.id == "under-served-DIM-05"
                && gap.dimension == Dimension::Dim05
                && gap.observed.expect("observed score").value() == 4.5
                && gap.source == GapSource::UnderServedRegion
        }));
    }

    #[test]
    fn find_gaps_reports_empty_dimension_without_manufacturing_score() {
        let mut entry = fully_scored_entry("corridor-1", 9.0);
        entry.scores.remove("DIM-08");

        let gaps = find_gaps(&[entry], &Rubric::default_v0()).expect("gap analysis succeeds");
        let gap = gaps
            .iter()
            .find(|gap| gap.id == "empty-DIM-08")
            .expect("empty dimension gap");

        assert_eq!(gap.observed, None);
        assert_eq!(gap.source, GapSource::EmptyRegion);
    }

    #[test]
    fn null_result_records_car_air_dominant_market_without_manufactured_gap() {
        let entry = fully_scored_entry("corridor-1", 10.0);

        let result = null_result("car-air-dominant-market", &[entry], &Rubric::default_v0())
            .expect("null-result evaluation succeeds")
            .expect("no gaps yields null result");

        assert_eq!(result.region_id, "car-air-dominant-market");
        assert_eq!(result.evidence_label, EvidenceLabel::Implemented);
        assert!(result.rationale.contains("No gap manufactured"));
    }

    #[test]
    fn null_result_is_absent_when_gap_exists() {
        let mut entry = fully_scored_entry("corridor-1", 10.0);
        entry.scores.insert("DIM-01".to_string(), 3.0);

        assert_eq!(
            null_result("not-null", &[entry], &Rubric::default_v0()),
            Ok(None)
        );
    }

    #[test]
    fn tier_sla_gap_is_integrated_as_gap_region() {
        let mut entry = fully_scored_entry("corridor-1", 10.0);
        entry.scores.insert("DIM-13".to_string(), 6.0);

        let gaps = find_gaps(&[entry], &Rubric::default_v0()).expect("gap analysis succeeds");

        assert!(gaps.iter().any(|gap| {
            gap.id == "tier-sla-corridor-1"
                && gap.dimension == Dimension::Dim13
                && gap.source == GapSource::TierSla
        }));
    }

    fn corpus_with_dim01(scores: &[f64]) -> Vec<CorpusEntry> {
        scores
            .iter()
            .enumerate()
            .map(|(idx, value)| {
                let mut entry = fully_scored_entry(&format!("corridor-{idx}"), 10.0);
                entry.scores.insert("DIM-01".to_string(), *value);
                entry
            })
            .collect()
    }

    #[test]
    fn tail_gap_isolates_concentrated_deficit_to_named_minority() {
        let corpus = corpus_with_dim01(&[10.0, 10.0, 7.0, 7.0, 7.0, 7.0, 4.0, 2.0]);
        let gaps = find_gaps(&corpus, &Rubric::default_v0()).expect("gap analysis runs");

        let tail = gaps
            .iter()
            .find(|gap| gap.id == "tail-DIM-01")
            .expect("tail region present");
        assert_eq!(tail.source, GapSource::TailRegion);
        assert_eq!(tail.entry_ids.len(), 2);
        assert!(tail.entry_ids.contains(&"corridor-6".to_string()));
        assert!(tail.entry_ids.contains(&"corridor-7".to_string()));

        let under = gaps
            .iter()
            .find(|gap| gap.id == "under-served-DIM-01")
            .expect("min region present");
        assert_eq!(under.entry_ids.len(), corpus.len());
    }

    #[test]
    fn adequate_distribution_has_no_tail_gap() {
        let corpus = corpus_with_dim01(&[8.0, 8.0, 9.0, 10.0]);
        let gaps = find_gaps(&corpus, &Rubric::default_v0()).expect("gap analysis runs");

        assert!(!gaps.iter().any(|gap| gap.source == GapSource::TailRegion));
    }

    #[test]
    fn majority_deficit_is_classified_systemic_not_tail() {
        // 4 of 6 entries below the 7.0 bar (67%): the bottom-quartile trigger
        // still fires, but the share crosses SYSTEMIC_SHARE so it is reported as
        // a systemic deficit rather than a concentrated tail.
        let corpus = corpus_with_dim01(&[10.0, 9.0, 2.0, 2.0, 1.0, 1.0]);
        let gaps = find_gaps(&corpus, &Rubric::default_v0()).expect("gap analysis runs");

        assert!(
            !gaps.iter().any(|gap| gap.source == GapSource::TailRegion),
            "a majority deficit must not be labelled a tail"
        );
        let systemic = gaps
            .iter()
            .find(|gap| gap.id == "systemic-DIM-01")
            .expect("systemic region present");
        assert_eq!(systemic.source, GapSource::SystemicRegion);
        assert_eq!(systemic.entry_ids.len(), 4);
        assert!(systemic.reason.contains("67%"));
    }
}
