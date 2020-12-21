use colored::*;

pub fn structs() {
    println!("\n{}", "Structs".green().bold());

    struct User {
        email: String,
    }

    // Immutable instance
    let user1 = User {
        email: String::from("someone@example.com"),
    };

    // Mutable instance
    let mut user2 = User {
        email: String::from("someone@example.com"),
    };

    user2.email = String::from("anotheremail@example.com");

    // Spread create
    let _user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // Tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    // We can split the implementation to multiple blocks
    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let _square1 = Rectangle::square(50);
}
