#[derive(Debug)]
struct Rectangle {
    length:u32,
    width:u32
}

// we can also have mulitple impl blocks

impl Rectangle {
    fn area(&self) -> u32 { // this self parameter is the instance of the struct the method is being called for here the struct is rect
        // this &self borrows the value not takes the ownership
        self.length*self.width
    }

    // this is associated function because it does require instance to call upon
    fn can_hold(&self, other:&Rectangle) -> bool {
        self.length>=other.length && self.width>=other.width
    }

    // this is non associated function because it does not require instance to call upon
    fn square(size:u32) -> Self {
        Self {
            length:size,
            width:size
        }
    }
}

fn main() {
    let rect = Rectangle {
        length:30,
        width:40
    };

    let rect2 = Rectangle {
        length:20,
        width:20
    };

    // println!("the area of ${rect:?} is {}",rect.area()); // this is called method syntax

    println!("can rect hold rect2 {}",rect.can_hold(&rect2));

    let s = Rectangle::square(30);

    println!("the area of the square rectangle is {s:?}");
}

// ---------------------Associated functions----------------//

// Associated functions are those functions which calls upon an instance and are inside the impl block 
// for eg rect.area() area is an associated functions because it has an instance rect

// String::from() this is not an associated function because they dont have an instance to call upon
// we can make our own non associated functions
