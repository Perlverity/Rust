fn add <T> (a: T, b: T) -> T
    where T: std::ops::Add<Output=T>
{
    a + b
}

fn x2 <T: std::ops::Add<Output=T> + Copy> (n: T) -> T {
    n + n
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> where T: std::ops::AddAssign {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn add(&mut self, pt: Point<T>) {
        self.x += pt.x;
        self.y += pt.y;
    }
}

fn main() {
    println!("{}", add(10, 25));
    println!("{}", add(10.0, 25.0));
    println!("{}", add::<i32>(10, 25));
    // println!("{}", add('a', 'a'));

    println!("{}", x2(3));
    println!("{}", x2(3.0));
    println!("{}", x2::<u64>(3));

    let pt_i = Point { x: 20, y: 50 };
    let pt_f = Point { x: 20.5, y: 15.3 };
    println!("{:?}", pt_i);
    println!("{:?}", pt_f);

    let mut pt = Point::new(10, 10);
    println!("{:?}", pt);
    pt.add(Point{ x: 20, y: 30});
    println!("{:?}", pt);
}
