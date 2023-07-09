mod linked_list;
pub use linked_list::bad_single::*;

pub fn test() {
    let list = List::new();
    println!("{:?}", list);
}
