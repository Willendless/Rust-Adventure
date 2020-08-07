#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width
            && self.height >= other.height
    }
}


fn main() {
    let width1 = 30;
    let height1 = 50;

    let dimensions: (u32, u32) = (30, 50);

    let rec1 = Rectangle {
        width: 30,
        height: 50
    };

    println!("The rec is {:?}", rec1);

    println!(
        "The area of the rectangle is: {} square pixels.
        {}, {}, {},
        A can hold B: {}",
        area(width1, height1),
        tuple_area(dimensions),
        struct_are(&rec1),
        rec1.area(),
        Rectangle::square(10).can_hold(&Rectangle::square(1))
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn struct_are(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}

