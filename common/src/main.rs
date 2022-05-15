
fn main() {
    println!("Hello, world!");

    let mut x = 5;
    println!("The value of x is {}: ", x);

    x = 6;
    println!("The value of x is {}: ", x);
    
    const THREE_HOUSE_IN_SECONDS :u32 = 60 * 60 * 3; 
    println!("THREE_HOUSE_IN_SECONDS: {} ", THREE_HOUSE_IN_SECONDS);

    let y = 6;
    
    println!("The value of y: {}" , y);

    let y = y + 5;

    println!("The value of y: {}" , y);
    {
        let y  = y * 2;

        println!("The value of y: {}" , y);
    }

    println!("The value of y: {}" , y);

    let mut spaces = "    ";
    spaces = spaces.len();

    println!("The value of space is : {}", spaces);
    
}

