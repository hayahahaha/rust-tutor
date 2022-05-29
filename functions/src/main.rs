fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {}", y);

    let mut x = get_five();


    println!("The value of x is {}", x);
    x = plus_one(x);

    println!("The value of x is {}", x);
    
}


fn another_function() {
    println!("this is another function");
}

fn print_value(value: u32) {
    println!("value is {}", value);
}

fn get_five() -> i32 {
    5
       
}

fn plus_one(x : i32) -> i32 {
    x + 1
}
