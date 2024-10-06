use std::fs;

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

fn main() {
    let text = fs::read_to_string("logs.txt").expect("file is not found");
    let error_logs = extract_errors(text.as_str());
    println!("{:#?}", error_logs)
}
