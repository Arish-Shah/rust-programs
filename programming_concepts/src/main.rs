fn main() {
    // variables_and_mutability();
    // data_types();
    functions();
}

fn variables_and_mutability() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const SOME_CONSTANT_VALUE: u32 = 12;
    println!("Constant value: {}", SOME_CONSTANT_VALUE);

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x is: {}", x);
    }
    println!("The value of x is: {}", x);

    let _spaces = "   ";
    let _spaces = _spaces.len();
}

fn data_types() {
    let integer: i32 = 10;
    let floating: f64 = 2.0;

    println!("Integer: {}", integer);
    println!("Floating: {}", floating);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value is y is: {}", y);

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    println!("2nd element of the array is: {}", arr[1]);
}

fn functions() {
    
}
