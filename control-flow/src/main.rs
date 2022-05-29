use std::io;

fn main() {
    for number in (1..4).rev() {
        println!("The number {}", number);
    }
    println!("Loop end!");
}

fn collection_loop() {
    let a: [i32; 4] = [1, 2, 3, 4];

    for element in a {
        println!("The value of eletment {}", element);
    }

    println!("End loop!");
}

fn conditional_loop() {
    let mut number = 3;

    while number != 0 {
        println!("The number is {}", number);
        number  -=1;

    }
    println!("End loop!");
}

fn return_value() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 1 {
            break counter *2;
        }
    };

    println!("The value of counter is {}", counter);
    println!("The value of result is {}", result);
}

fn break_loop () {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9  {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -=1;
        }
        count += 1;
    }
    println!("End count = {}", count);

}

fn run_loop() {
    loop {
        println!("Loop!");
    }
}

fn isEven() {

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Fail to read_line");

    println!("The value of input is = {}", input);

    let x :u32 = input.trim().parse().expect("It not a number");
    
    if  x % 2 == 0 {
        println!("Hello, world!");
        println!("even");
    } else {
        println!("not even");
    }
}
