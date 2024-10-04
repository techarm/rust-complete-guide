#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(i32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        // if let Media::Book { title, author } = self {
        //     format!("Book: {}, {}", title, author)
        // } else if let Media::Movie { title, director } = self {
        //     format!("Movie: {}, {}", title, director)
        // } else if let Media::Audiobook { title } = self {
        //     format!("Audiobook: {}", title)
        // } else {
        //     String::from("Media description")
        // }

        match self {
            Media::Book { title, author } => {
                format!("Book: {}, {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {}, {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
            Media::Podcast(id) => {
                format!("Podcast: {}", id)
            }
            Media::Placeholder => String::from("Placeholder"),
        }
    }
}

#[derive(Debug)]
struct Catalog {
    medias: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { medias: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.medias.push(media);
    }

    fn summary(&self) -> Vec<String> {
        self.medias
            .iter()
            .map(|media| media.description())
            .collect::<Vec<String>>()
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

    // println!("{}", book.description());
    // println!("{}", movie.description());
    // println!("{}", audio_book.description());

    let mut catalog = Catalog::new();
    catalog.add(book);
    catalog.add(movie);
    catalog.add(audio_book);
    catalog.add(Media::Podcast(10));
    catalog.add(Media::Placeholder);

    // Options: None and Some
    for i in 0..=5 {
        match catalog.medias.get(i) {
            Some(media) => {
                println!("Get data: {:?}", media);
            }
            None => {
                println!("Not found data")
            }
        }
    }

    println!("{:#?}", catalog);
    println!("{:#?}", catalog.summary());
}
