pub fn main(){
    let width = 30;
    let length = 50;

    let area = area(length, width);
    println!("{}", area);

    let area = area_tuples((length, width));
    println!("{}", area);

    let scale = 2;
    let rectangle = Rectangle{
        length: dbg!(32 * scale), 
        width: 100
    };

    let area = area_structs(&rectangle);
    println!("{}", area);

    dbg!(&rectangle);
    println!("{:#?}", rectangle);


}

#[derive(Debug)]
struct Rectangle {
    length: i32,
    width: i32
}


fn area(l: i32, w: i32) -> i32 {
    return l * w;
}

fn area_tuples(dimensions: (i32, i32)) -> i32 {
    return dimensions.0 * dimensions.1;
}

fn area_structs(rectangle: &Rectangle) -> i32 {
    return rectangle.length * rectangle.width;
}