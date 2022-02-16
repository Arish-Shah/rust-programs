fn main() {
    // Variables and Mutability
    variables_and_mutability();
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

    let spaces = "   ";
    let spaces = spaces.len();
}
