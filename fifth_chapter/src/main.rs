// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
// struct AlwaysEqual;

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn width(&self) -> bool {
//         self.width > 0
//     }    

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }

//     fn square(size: u32) -> Self {
//         Self {
//             width: size,
//             height: size,
//         }
//     }
// }



fn main() {
    // let user1 = User {
    //     active:true,
    //     username: String::from("someusername123"),
    //     email:String::from("someone@example.com"),
    //     sign_in_count:1,
    // }
    // let black = Color(0,0,0);
    // let origin = Point(0,0,0);

    // let subject = AlwaysEqual;

    // let width1 = 30;
    // let height1 = 50;

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );
    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(&rect1)
    // );
    // println!("rect1 is {:?}", rect1);


    // let scale = 2;
    // let rect1 = Rectangle {
    //     width: dbg!(30 * scale),
    //     height: 50,
    // };

    // dbg!(&rect1);


    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };


    // if rect1.width(){
    //     println!(
    //         "The area of the rectangle is {} square pixels.",
    //         rect1.area()
    //     );
    // }


}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// struct User {
//     active:bool,
//     username:String,
//     email:String,
//     sign_in_count:u64,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }