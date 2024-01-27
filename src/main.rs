fn main() {

    let y = 42;
    let mut x = 10;
    let t = (4, 'a');
    let (a, b) = t;
    println!("a = {}", a);
    let a = [10, 11, 12, 13, 14, 15];
    let mut x = a[2];
    println!("{}", x);
    fn area(w: i32, h: i32) -> i32 {
        w * h
    }
    fn hello_world(){
        println!("Hello, world!");
    }
    hello_world();

    if x < 5 {
        println!("number was less than five");
    } else {
        println!("number was not less than five");
    }
    fn function_returning_boolean()-> bool{
        true
    }
    let y = if function_returning_boolean() {
        42
    } else {
        7
    };
    println!("{}",y);

    while x > 0 {
        println!("{}", x);
        x = x - 1;
    }

    for e in [10, 20, 30, 40, 50] {
        println!("the value is: {}", e);
    }
    //ex 1,2,3
    #[derive(Debug)]
    enum Op {
        Transparent,
        Opaque
    }
    struct Rectangle {
        width: u32,
        height: u32,
        seethrough: Op
    }
    struct Point<T> {
        x: T,
        y: T,
    }
    impl Rectangle {
        fn area(self) -> u32 {
            self.width * self.height
        }
        fn covers(&self) -> bool {
            match self.seethrough {
                Op::Opaque => true,
                Op::Transparent => false
            }
        }
        fn in_box(&self, x:u32,y:u32)-> bool{
            if self.height > x && self.width > y{ //expression always evaluates to true I think
                true
            } else { false }
        }
        fn width(&self)->u32{
            self.width
        }
    }

    let intpoint = Point::<i32>{ x: 5, y: 10 };
    let mut rect = Rectangle{width: intpoint.x as u32, height: intpoint.y as u32, seethrough: Op::Transparent };
    rect = Rectangle{width: intpoint.x as u32, height: intpoint.y as u32, seethrough: Op::Transparent };
    println!("{},{}", intpoint.x, intpoint.y);
    println!("{}",rect.area());
    rect = Rectangle{width: 1, height: 1, seethrough: Op::Opaque };
    //let b = rect.covers(3, 8, intpoint.x,intpoint.y);
    let mut b = &rect.covers();
    if *b{
        let b = &rect.in_box(intpoint.x as u32, intpoint.y as u32);
    }
    println!("{}",*b);

    //ex 4 5 6 7
    let height = [2,5,7,9,10];
    let width = [1,3,5,7,9];
    let p = Point::<i32>{x:5,y:5};
    let mut iterator = 0;
    while iterator < height.len() {
        let r = Rectangle{height: height[iterator] as u32,width:width[iterator],seethrough: Op::Opaque};
        if r.in_box(p.x as u32, p.y as u32){
            println!("It's in the box")
        } else { println!("Not in the box") }
        iterator += 1;
    }
    fn enlarge(mut r: Rectangle) -> Rectangle {
        r.width = r.width*2;
        r
    }
    let r1 = Rectangle {width: 5, height: 5, seethrough: Op::Opaque };
    let r2 = enlarge(r1);
    println!("r2.width() = {}", r2.width());
}
