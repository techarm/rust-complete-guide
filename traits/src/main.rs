mod bucket;
mod container;
mod stack;

use bucket::Bucket;
use container::Container;
use stack::Stack;

fn main() {
    let mut b = Bucket::new(String::from("hi here"));
    println!("{}", b.is_empty());
    println!("{:?}", b.get());
    println!("{}", b.is_empty());

    b.put(String::from("new item"));
    println!("{:?}", b.is_empty());

    let mut s = Stack::new(vec![String::from("1"), String::from("2")]);
    println!("{:?}", s.get());

    add_string(&mut b, String::from("hello"));
    add_string(&mut s, String::from("3"));
}

fn add_string<T: Container<String>>(c: &mut T, item: String) {
    c.put(item);
}
