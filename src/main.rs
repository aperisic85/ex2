
#[derive(Debug)]
struct Point(i32,i32);

fn add(p1: &Point, p2: &Point) -> Point {

    Point(p1.0 + p2.0, p1.1 + p2.1)
}
fn main() {

    let p1 = Point(4,9);
    let p2 = Point(2,12);
    let p3 = add(&p1,&p2);
    println!("{p1:?} + {p2:?} = {p3:?}")


}
