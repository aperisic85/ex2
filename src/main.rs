
#[derive(Debug)]
struct Point(i32,i32);

fn add(p1: &Point, p2: &Point) -> Point {

    let p = Point(p1.0 + p2.0, p1.1 + p2.1);
    //print struct stack address 
    println!("temp struct stack address (inside function)->: {:p}", &p.0);
    p
}
fn main() {

    let p1 = Point(4,9);
    let p2 = Point(2,12);
    let p3 = add(&p1,&p2);
    println!("{p1:?} + {p2:?} = {p3:?}");
    println!("p3 stack address-> {:p}", &p3.0);


}
