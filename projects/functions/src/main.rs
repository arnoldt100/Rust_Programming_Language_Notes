fn main() {
    println!("Hello, world!");

    let y :i32 = another_function();
    println!("The value of y in main is {y}");

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    }
    else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


}

fn another_function()->i32 {
    let y :i32 = {
        let x=3;
        x+1
    };
    println!("The value of y in another_function is {y}");
    y
}

