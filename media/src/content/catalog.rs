use super::media::Media;

#[derive(Debug)]
pub struct Catalog {
    pub medias: Vec<Media>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog { medias: vec![] }
    }

    pub fn add(&mut self, media: Media) {
        self.medias.push(media);
    }

    pub fn summary(&self) -> Vec<String> {
        self.medias
            .iter()
            .map(|media| media.description())
            .collect::<Vec<String>>()
    }
}
