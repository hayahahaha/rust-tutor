use std::io;
fn main() {    
    let t = true;
    
    let f : bool = false;

    println!("hello {}", t);

    println!("hello 2 {}", f);
    let a = 10 + 20;
    println!("hello world {}", a);
    

    let tup: (i32, f64, u8) =  (500, 6.4, 1);
    let (x, y ,z ) = tup;
    println!(" the value of y = {}", y);

    let a = tup.0;
    println!(" the value of a  = {}", a);
    
    let b = [1, 2, 3];
    println!("the value of b {}", b[0]);

    let arr: [i32 ; 4] = [1, 2, 3, 4];
    println!("the value of arr {} {} ", arr[0], arr[2]);
    
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Fail to read_line");

    let index: usize = index.trim().parse().expect("index entered was not a number ");
    
    let element = arr[index];

    println!("the value of the element at index {} is : {}", index, element);
    
    
}


