use std::io;

fn main() {
    // VARIABLES AND MUTABILITY

    // let mut x = 5;
    // println!("The value of x is {x}");

    // x = 7;
    // println!("The new value of x is {x}");


    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("3 Hours in seconds are {THREE_HOURS_IN_SECONDS}");

    // let x = 5;
    // let x = x+1;

    // {
    //     let x = x*2;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    // println!("The value of x in the outer scope is {x}");


    // let spaces = "  ";
    // let spaces = spaces.len();

    // println!("Spaces has a length of: {}", spaces);


    // let mut spaces = "      ";
    // spaces = spaces.len();
    // println!("Spaces has a length of {}", spaces);



    // DATA TYPES
    // let guess: u32 = "42".parse().expect("Not a number!");
    // let x = 2.0; // f64
    // let y:f32 =3.0; // f32

    // //Adition
    // let sum = 5+10;
    // // subtraction
    // let difference = 95.5 - 4.3;
    // // multiplication
    // let product = 4*30;
    // // division
    // let quotient = 56.7 / 32.3;
    // let truncated = -5 / 3; // Results in -1 
    // // remainder 
    // let remainder = 45 % 5;

    // let t = true;
    // let f:bool = false; // with explicit  type annotation


    // let c = 'z';
    // let z:char = 'Z'; // with explicit type annotation
    // let heart_eyed_cat = '😻';

    // let tup: (i32, f64, u8) = (500,6.4,1);
    // let tup = (500,6.4,1);
    // let (x, y, z) = tup;
    // println!("The value of y is: {y}");

    // let five_hundred = x.0;
    // let six_point_four = x.1;
    // let one = x.2;


    // let a = [1,2,3,4,5];

    // let months = ["January", "February", "March", "April", "May", "June", "July",
    // "August", "September", "October", "November", "December"];

    // let a:[i32,5] = [1,2,3,4,5];
    // let a = [3;5];

    // let a = [1,2,3,4,5];
    // let first = a[0];
    // let second = a[1];

    // let a = [1,2,3,4,5];

    // println!("Please enter an array index.");
    

    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");
    

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");
    
    // let element = a[index];

    // println!("The value of the element at index{index} is: {element}");




    // FUNCTIONS

    // println!("HELLO WORLD");
    
    // another_function(121);
    // print_labeled_measurement(5,'h');

    // let y = {
    //     let x = 3;
    //     x + 1
    // };
    // println!("The value of y is: {y}");
    // let x = five();
    // println!("The value of x is: {x}");

    // let x = plus_one(5);
    // println!("The value of x is: {x}");

    let number = 3;

    // if number < 5 {
    //     println!("condiction was true");
    // } else {
    //     println!("condiction was false");
    // }


    // if number != 0 {
    //     println!("The number is different than 0");
    // }


    // if number % 4 == 0 {
    //     println!("Number is divisible by 4");
    // } else if number % 3 ==0 {
    //     println!("Number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("Number is divisible by 2");
    // } else {
    //     println!("Number is not divisible by 4,3,2");
    // }


    // let condition = true;
    // let number = if condition{ 5 } else { 6 };

    // println!("The value of number is: {number}");
    


    // let mut counter = 0;
    // let result = loop {
    //     println!("Inside the loop {counter}");
    //     if counter == 10 {
    //         break counter *2;
    //     }
    //     counter += 1;
    // };
    // println!("Outside the loop {result}");


    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
        
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
        
    //         if remaining == 9 {
    //             break;
    //         }

    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -=1;
    //     }
    //     count += 1;
    // }
    // println!("End count = {count}");


    // let mut number = 3;
    // while number != 0 {
    //     println!("{number}");

    //     number -= 1;
    // }
    // println!("LITOFF!!!");


    // let a = [10,20,30,40,50];
    // let mut index = 0;
    // while index < 5 { 
    //     println!("The value is: {}", a[index]);
    //     index += 1;
    // }

    // let a = [10,20,30,40,50];
    // for element in a { 
    //     println!("The value is {element}");
    // }

    // for number in (1..4).rev() {
    //     println!("{number}");
    // }
    // println!("LITOFF!!!");

}
// fn plus_one(x:i32) -> i32 {
//     x + 1
// }
/* 
fn five() -> i32 {
    5
}
 */

/* fn print_labeled_measurement(value:i32, unit_label:char) { 
    println!("The measurement is : {value}{unit_label}");
}


fn another_function (x:i64){
    println!("I DONT KNOW JAMBO: {x}");
}
 */