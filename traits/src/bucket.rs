use super::container::Container;

pub struct Bucket<T> {
    item: Option<T>,
}

impl<T> Bucket<T> {
    pub fn new(item: T) -> Self {
        return Bucket { item: Some(item) };
    }
}

impl<T> Container<T> for Bucket<T> {
    fn get(&mut self) -> Option<T> {
        self.item.take()
    }

    fn put(&mut self, item: T) {
        self.item = Some(item);
    }

    fn is_empty(&self) -> bool {
        self.item.is_none()
    }
}
