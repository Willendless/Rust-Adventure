struct CustomSmartPointer {
    data: String,
}

impl CustomSmartPointer {
    fn new(s: &str) -> CustomSmartPointer {
        CustomSmartPointer {
            data: String::from(s),
        }
    }
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data)
    }
}

fn main() {
    let c = CustomSmartPointer::new("a");
    let d = CustomSmartPointer::new("b");
    println!("CustomSmartPointers created.");
}
