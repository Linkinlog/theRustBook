#[derive(Debug)]
struct Rectangle {
    length: usize,
    width: usize,
}

// Method syntax
impl Rectangle {
    fn area(&self) -> usize {
        self.width * self.length
    }
}
// Non-Method syntax
fn get_area(rec: &Rectangle) -> usize {
    rec.length * rec.width
}
fn main() {
    let rec = Rectangle {
        length: 50,
        width: 30,
    };
    println!(
        "Area of {:?} without method syntax: {}",
        &rec,
        get_area(&rec)
    );
    println!(
        "Area of {:?} with method syntax: {}",
        &rec,
        &rec.area()
    );
}
