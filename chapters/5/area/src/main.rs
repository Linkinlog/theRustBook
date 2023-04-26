struct Rectangle {
    length: usize,
    width: usize,
}
fn get_area(rec: &Rectangle) -> usize {
    rec.length * rec.width
}
fn main() {
    let rec = Rectangle {
        length: 50,
        width: 30,
    };
    println!("{}", get_area(&rec));
}
