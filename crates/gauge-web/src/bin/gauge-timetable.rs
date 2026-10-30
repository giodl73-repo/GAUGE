fn main() {
    let result = std::env::args()
        .nth(1)
        .ok_or_else(|| "usage: gauge-timetable <query.json>".to_string())
        .and_then(|path| std::fs::read_to_string(path).map_err(|error| error.to_string()))
        .and_then(|input| gauge_web::timetable::inspect_timetable_json(&input));
    match result {
        Ok(output) => println!("{output}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(2);
        }
    }
}
