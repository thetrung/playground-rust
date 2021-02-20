
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

//
// impl block could be separated like partial C# class implement.
//
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    
    let rect = Rectangle { 
        width: 50, 
        height: 50
    };

    let rect2 = Rectangle { 
        width: 150, 
        height: 250
    };

    println!("\nThe area of {:?} rectangle is {} square pixels.",rect, rect.area());
    // self.area() was already ref in function.

    println!("\nIs {:?} can hold {:?} = {}\n", rect2, &rect, rect2.can_hold(&rect));
    // good example of reference/borrowing..
}
