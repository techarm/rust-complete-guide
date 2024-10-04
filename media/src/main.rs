#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}

impl Media {
    fn description(&self) -> String {
        if let Media::Book { title, author } = self {
            format!("Book: {}, {}", title, author)
        } else if let Media::Movie { title, director } = self {
            format!("Movie: {}, {}", title, director)
        } else if let Media::Audiobook { title } = self {
            format!("Audiobook: {}", title)
        } else {
            String::from("Media description")
        }
    }
}

fn main() {
    let book = Media::Book {
        title: String::from("A Book"),
        author: String::from("techarm"),
    };
    let movie = Media::Movie {
        title: String::from("A Movie"),
        director: String::from("techarm"),
    };
    let audio_book = Media::Audiobook {
        title: String::from("A AudioBook"),
    };

    println!("{}", book.description());
    println!("{}", movie.description());
    println!("{}", audio_book.description());
}
