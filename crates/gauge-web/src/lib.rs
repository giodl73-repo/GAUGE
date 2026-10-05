//! Browser adapter for the committed historical frequency corpus.
use gauge_corpus::CorpusEntry;
use gauge_score::{frequency_score, Dimension, Rubric, Score};
use serde::{Deserialize, Serialize};
const FIXTURES: &[(&str, &str)] = &[
    (
        "us-california-zephyr",
        include_str!("../../../corpus/us-california-zephyr.md"),
    ),
    (
        "us-capitol-corridor",
        include_str!("../../../corpus/us-capitol-corridor.md"),
    ),
    (
        "us-cascades",
        include_str!("../../../corpus/us-cascades.md"),
    ),
    (
        "us-crescent",
        include_str!("../../../corpus/us-crescent.md"),
    ),
    (
        "us-empire-builder",
        include_str!("../../../corpus/us-empire-builder.md"),
    ),
    (
        "us-hiawatha",
        include_str!("../../../corpus/us-hiawatha.md"),
    ),
    (
        "us-keystone",
        include_str!("../../../corpus/us-keystone.md"),
    ),
    (
        "us-nec-acela",
        include_str!("../../../corpus/us-nec-acela.md"),
    ),
    (
        "us-nec-regional",
        include_str!("../../../corpus/us-nec-regional.md"),
    ),
    (
        "us-pacific-surfliner",
        include_str!("../../../corpus/us-pacific-surfliner.md"),
    ),
    (
        "us-sunset-limited",
        include_str!("../../../corpus/us-sunset-limited.md"),
    ),
    (
        "us-wolverine",
        include_str!("../../../corpus/us-wolverine.md"),
    ),
];
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub corridor: usize,
    pub round_trips: f64,
    pub bar: f64,
}
#[derive(Debug, Serialize)]
pub struct Corridor {
    pub slug: String,
    pub name: String,
    pub termini: Vec<String>,
    pub tier: Option<String>,
    pub historical_trips: f64,
    pub trips: f64,
    pub score: f64,
    pub below_bar: bool,
    pub historical_source_id: String,
    pub frequency_basis: String,
}
#[derive(Debug, Serialize)]
pub struct Run {
    pub corridors: Vec<Corridor>,
    pub below: usize,
    pub classification: String,
}
#[derive(Debug, Serialize)]
pub struct Output {
    pub model: &'static str,
    pub input: Input,
    pub baseline: Run,
    pub scenario: Run,
}
fn corpus() -> Result<Vec<CorpusEntry>, String> {
    FIXTURES
        .iter()
        .map(|(_, text)| {
            CorpusEntry::from_markdown(&text.replace("\r\n", "\n")).map_err(|e| e.to_string())
        })
        .collect()
}
fn run(corpus: &[CorpusEntry], bar: f64) -> Result<Run, String> {
    let thresholds = std::collections::BTreeMap::from([(
        Dimension::Dim07,
        Score::new(bar).map_err(|e| e.to_string())?,
    )]);
    let gaps = gauge_gap::find_gaps_with_thresholds(corpus, &Rubric::default_v0(), &thresholds)
        .map_err(|e| e.to_string())?;
    let classification = gaps
        .iter()
        .find(|g| {
            g.dimension == Dimension::Dim07
                && matches!(
                    g.source,
                    gauge_gap::GapSource::SystemicRegion | gauge_gap::GapSource::TailRegion
                )
        })
        .map(|g| format!("{:?}", g.source))
        .unwrap_or_else(|| "No frequency dispersion gap".into());
    let corridors = corpus
        .iter()
        .zip(FIXTURES)
        .map(|(entry, (slug, text))| {
            let quantity = entry
                .quantities
                .iter()
                .find(|q| q.unit == "round-trips-per-day")
                .ok_or("Missing frequency")?;
            let historical = CorpusEntry::from_markdown(&text.replace("\r\n", "\n"))
                .map_err(|e| e.to_string())?;
            let score = *entry.scores.get("DIM-07").ok_or("Missing DIM-07")?;
            Ok(Corridor {
                slug: (*slug).into(),
                name: text
                    .lines()
                    .find_map(|line| line.strip_prefix("# "))
                    .ok_or("Missing name")?
                    .into(),
                termini: entry.termini.clone(),
                tier: entry.tier.clone(),
                historical_trips: historical.quantities[0].value,
                trips: quantity.value,
                score,
                below_bar: score < bar,
                frequency_basis: if quantity.source_id.is_some() {
                    "historical"
                } else {
                    "hypothetical"
                }
                .into(),
                historical_source_id: historical.quantities[0]
                    .source_id
                    .clone()
                    .ok_or("Missing source")?,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let below = corridors.iter().filter(|c| c.below_bar).count();
    Ok(Run {
        corridors,
        below,
        classification,
    })
}
pub fn evaluate(input: Input) -> Result<Output, String> {
    if input.corridor >= FIXTURES.len()
        || !input.round_trips.is_finite()
        || !(0. ..=32.).contains(&input.round_trips)
        || !input.bar.is_finite()
        || !(0. ..=10.).contains(&input.bar)
    {
        return Err("Choose a known corridor, 0-32 round trips/day and a 0-10 bar".into());
    }
    let mut corpus = corpus()?;
    let baseline = run(&corpus, input.bar)?;
    let selected = &mut corpus[input.corridor];
    selected.scores.insert(
        "DIM-07".into(),
        frequency_score(input.round_trips)
            .map_err(|e| e.to_string())?
            .value(),
    );
    selected.quantities[0].value = input.round_trips;
    selected.quantities[0].label = Some(gauge_corpus::EvidenceLabel::Simulated);
    selected.quantities[0].source_id = None;
    let scenario = run(&corpus, input.bar)?;
    Ok(Output {
        model: "gauge-frequency-v1",
        input,
        baseline,
        scenario,
    })
}
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub fn evaluate_json(json: &str) -> Result<String, String> {
    if json.len() > 8192 {
        return Err("Scenario exceeds 8 KB".into());
    }
    serde_json::to_string(&evaluate(
        serde_json::from_str(json).map_err(|e| e.to_string())?,
    )?)
    .map_err(|e| e.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn historical_baseline_and_changed_membership() {
        let out = evaluate(Input {
            corridor: 0,
            round_trips: 16.,
            bar: 7.,
        })
        .unwrap();
        assert_eq!(out.baseline.below, 8);
        assert_eq!(out.scenario.below, 7);
        assert_eq!(out.baseline.classification, "SystemicRegion");
        assert_eq!(out.scenario.corridors[0].score, 10.);
        assert_eq!(out.baseline.corridors.len(), 12);
    }
    #[test]
    fn exact_bar_agrees_with_displayed_membership() {
        let out = evaluate(Input {
            corridor: 0,
            round_trips: 5.8,
            bar: 3.6,
        })
        .unwrap();
        assert_eq!(out.scenario.corridors[0].score, 3.6);
        assert!(!out.scenario.corridors[0].below_bar);
        assert_eq!(out.scenario.below, 5);
        assert_eq!(out.scenario.classification, "TailRegion");
    }
    #[test]
    fn invalid_and_boundary_inputs() {
        for v in [-1., 33., f64::NAN] {
            assert!(evaluate(Input {
                corridor: 0,
                round_trips: v,
                bar: 7.
            })
            .is_err());
        }
        assert!(evaluate_json("{}").is_err());
        assert!(evaluate_json(&"x".repeat(8193)).is_err());
        assert_eq!(
            evaluate(Input {
                corridor: 0,
                round_trips: 0.,
                bar: 0.
            })
            .unwrap()
            .scenario
            .below,
            0
        );
        assert_eq!(frequency_score(32.).unwrap().value(), 10.);
    }
    #[test]
    fn corpus_frequency_transform_matches_stored_scores() {
        for entry in corpus().unwrap() {
            assert_eq!(
                frequency_score(entry.quantities[0].value).unwrap().value(),
                entry.scores["DIM-07"]
            );
        }
    }
}
