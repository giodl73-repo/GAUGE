//! Frozen document observations, independent of the historical model corpus.
use serde::{Deserialize, Serialize};
use serde_json::json;
const SOURCE: &str = include_str!("../../../data/cascades-2023/rows.json");
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum Direction {
    Southbound,
    Northbound,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Query {
    direction: Direction,
    #[serde(default)]
    include_long_distance: bool,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Row {
    direction: Direction,
    page: u8,
    column: u8,
    train_number: Option<String>,
    long_distance: bool,
    departure_minute: u16,
    arrival_minute: u16,
    departure_printed: String,
    arrival_printed: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Dataset {
    schema: String,
    effective_date: String,
    source_sha256: String,
    calendar: String,
    rows: Vec<Row>,
}
fn source() -> Result<Dataset, String> {
    let d: Dataset = serde_json::from_str(SOURCE).map_err(|e| e.to_string())?;
    if d.schema != "gauge.cascades-schedule.v1" || d.rows.len() != 14 {
        return Err("Invalid frozen source shape".into());
    }
    for direction in [Direction::Southbound, Direction::Northbound] {
        let rows: Vec<_> = d.rows.iter().filter(|r| r.direction == direction).collect();
        if rows.len() != 7
            || rows.iter().enumerate().any(|(i, r)| {
                usize::from(r.column) != i + 1
                    || r.departure_minute > 1439
                    || r.arrival_minute > 1439
                    || r.arrival_minute < r.departure_minute
            })
        {
            return Err("Invalid frozen source columns or clock minutes".into());
        }
    }
    Ok(d)
}
fn inspect(query: Query) -> Result<serde_json::Value, String> {
    let data = source()?;
    let mut rows: Vec<_> = data
        .rows
        .into_iter()
        .filter(|r| {
            r.direction == query.direction && (query.include_long_distance || !r.long_distance)
        })
        .collect();
    rows.sort_by_key(|r| r.departure_minute);
    let first = rows.first().ok_or("Empty frozen schedule selection")?;
    let last = rows.last().ok_or("Empty frozen schedule selection")?;
    let span = last.departure_minute - first.departure_minute;
    let largest_gap = rows
        .windows(2)
        .map(|r| r[1].departure_minute - r[0].departure_minute)
        .max();
    Ok(json!({
        "schema":"gauge.scheduled-departures.v1", "direction":query.direction,
        "include_long_distance":query.include_long_distance,
        "source":{"effective_date":data.effective_date,"sha256":data.source_sha256,"pdf":"schedule/schedule.pdf","calendar":data.calendar,"host_ownership_verified":false},
        "segment":if query.direction == Direction::Southbound {"Seattle-Portland"} else {"Portland-Seattle"},
        "scheduled_departures":rows.len(), "first_departure":first.departure_printed,"last_departure":last.departure_printed,
        "departure_span_minutes":span,"largest_adjacent_gap_minutes":largest_gap,
        "round_trips":null,"overnight_gap_minutes":null,"actual_operations_verified":false,
        "historical_seed_quantities_verified":0,"original_model_changed":false,
        "authority":{"current_timetable":false,"booking":false,"sla":false,"host_capacity":false,"national_inference":false,"fiscal_admission":false},
        "scope":"Printed daily local-clock station-pair pattern, not verified operation or full Vancouver-Eugene service; unnumbered slots remain unidentified.",
        "rows":rows
    }))
}
/// Select frozen source rows. Never accepts new times or a live service date.
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub fn inspect_timetable_json(input: &str) -> Result<String, String> {
    if input.len() > 4096 {
        return Err("Schedule query exceeds 4 KiB".into());
    }
    let query: Query = serde_json::from_str(input).map_err(|e| e.to_string())?;
    serde_json::to_string(&inspect(query)?).map_err(|e| e.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn directional_scope_and_within_day_metrics_are_distinct() {
        for (direction, count, span, gap) in
            [("southbound", 6, 838, 302), ("northbound", 6, 760, 225)]
        {
            let d: serde_json::Value = serde_json::from_str(
                &inspect_timetable_json(&format!(r#"{{"direction":"{direction}"}}"#)).unwrap(),
            )
            .unwrap();
            assert_eq!(d["scheduled_departures"], count);
            assert_eq!(d["departure_span_minutes"], span);
            assert_eq!(d["largest_adjacent_gap_minutes"], gap);
            assert!(d["round_trips"].is_null());
            assert!(d["overnight_gap_minutes"].is_null());
            assert_eq!(d["actual_operations_verified"], false);
            assert_eq!(
                d["rows"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|r| r["train_number"].is_null())
                    .count(),
                1
            );
        }
    }
    #[test]
    fn long_distance_columns_are_an_explicit_choice() {
        for (direction, gap) in [("southbound", 200), ("northbound", 210)] {
            let d: serde_json::Value = serde_json::from_str(
                &inspect_timetable_json(&format!(
                    r#"{{"direction":"{direction}","include_long_distance":true}}"#
                ))
                .unwrap(),
            )
            .unwrap();
            assert_eq!(d["scheduled_departures"], 7);
            assert_eq!(d["largest_adjacent_gap_minutes"], gap);
            assert_eq!(
                d["rows"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|r| r["long_distance"] == true)
                    .count(),
                1
            );
            assert!(d["authority"]
                .as_object()
                .unwrap()
                .values()
                .all(|v| v == false));
        }
    }
    #[test]
    fn dates_new_values_unknown_fields_and_oversize_queries_fail_closed() {
        for q in [
            r#"{}"#,
            r#"{"direction":"eastbound"}"#,
            r#"{"direction":"southbound","date":"2024-01-01"}"#,
            r#"{"direction":"southbound","include_long_distance":"yes"}"#,
            r#"{"direction":"southbound","departure_minute":0}"#,
        ] {
            assert!(inspect_timetable_json(q).is_err());
        }
        assert!(inspect_timetable_json(&"x".repeat(4097)).is_err());
    }
}
