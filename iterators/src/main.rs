fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("yellow"),
        String::from("green"),
    ];

    // let mut iter = colors.iter();
    // println!("{:#?}", iter.next());
    // println!("{:#?}", iter.next());
    // println!("{:#?}", iter.next());
    // println!("{:#?}", iter.next());

    // print_element(&colors);
    // shorten_strings(&mut colors);
    // print_element(&colors);

    // let mut dest = vec![];
    // move_elements(colors, &mut dest);
    // println!("{:#?}", dest);
    // println!("{:#?}", colors);

    // print!("explode: {:#?}", explode(&colors))

    println!("find_color_or: {}", find_color_or(&colors, "re", "blue"));
}

fn print_element(items: &Vec<String>) {
    items
        .iter()
        // .map(|ele| format!("{} {}", ele, ele))
        .for_each(|ele| println!("{}", ele));
}

fn shorten_strings(items: &mut Vec<String>) {
    items.iter_mut().for_each(|el| el.truncate(1))
}

fn to_uppercase(items: &[String]) -> Vec<String> {
    items
        .iter()
        .map(|el| el.to_uppercase())
        .collect::<Vec<String>>()
}

fn move_elements(src: Vec<String>, dest: &mut Vec<String>) {
    src.into_iter().for_each(|el| dest.push(el));
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|el| el.chars().map(|c| c.to_string()).collect())
        .collect()
}

fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements
        .iter()
        .find(|el| el.starts_with(search))
        .map_or(fallback.to_string(), |el| el.to_string())
}
