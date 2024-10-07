fn main() {
    let langs = vec![
        String::from("rust"),
        String::from("go"),
        String::from("typescript"),
    ];

    let result = next_lang(&langs, "go");

    println!("{}", result);
}

fn next_lang<'a>(langs: &'a [String], current: &str) -> &'a str {
    let mut is_found = false;
    for lang in langs {
        if is_found {
            return lang;
        }
        if lang == current {
            is_found = true;
        }
    }
    langs.last().unwrap()
}
