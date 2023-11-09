pub fn main() -> () {
    println!("\n -- Methods -- \n");
    let rectangle = Rectangle {
        width: 32,
        length: 11,
    };

    println!("{:?}", rectangle);
    println!("The area of the rectangle with length {} and width {} is {}", rectangle.length, rectangle.width, rectangle.area());

    println!("{}", rectangle.width);
    println!("{}", rectangle.width());


    let rect1 = Rectangle {
        width: 10,
        length: 10,
    };

    let rect2 = Rectangle {
        width: 5,
        length: 5,
    };

    let rect3 = Rectangle { width: 3, length: 17 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::sqaure(33);
    println!("{:?}", rect4);


}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    length: i32
}


impl Rectangle {
    fn area(&self) -> i32 {
        return self.width * self.length;
    }
    
    fn width(&self) -> i32 {
        return self.width;
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        return rect.width < self.width && rect.length < self.width;
    }

    fn sqaure(size: i32) -> Self {
        return Self { width: size, length: size };
    }
}