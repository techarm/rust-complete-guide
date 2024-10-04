mod content;

use content::catalog::Catalog;
use content::media::Media;

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
