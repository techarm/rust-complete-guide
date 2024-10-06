use std::fs;
use std::io::Error;

fn extract_errors(text: &str) -> Vec<&str> {
    let lines = text.split("\n");

    let mut results = vec![];
    for line in lines {
        if line.starts_with("ERROR") {
            results.push(line);
        }
    }

    results
}

fn main() -> Result<(), Error> {
    let text = fs::read_to_string("logs.txt")?;
    let error_logs = extract_errors(text.as_str());
    fs::write("error_logs.txt", error_logs.join("\n"))?;
    Ok(())
}
