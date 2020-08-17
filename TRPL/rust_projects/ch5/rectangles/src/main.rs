#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let rect2 = (30, 50);

    let rect3 = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area of the rectangle1 is {} square pixels.",
        area1(width1, height1)
    );

    println!(
        "The area of the rectangle2 is {} square pixels.",
        area2(rect2)
    );

    println!("rect3 is {:?}", rect3);
    println!("rect3 is {:#?}", rect3);
    println!(
        "The area of the rectangle3 is {} square pixels.",
        area3(&rect3)
    );
    println!(
        "The area of the rectangle3 is {} square pixels.",
        rect3.area()
    );
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}

fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
