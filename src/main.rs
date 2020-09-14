use std::io;

fn main() {
    let h = gFFI(&"How precise would you like to be?");
    let mut limit: f64 = 2.0 / h;

    println!("Output was: {}", foo(limit, h));
}

fn gFFI(x: &str) -> f64 {
    println!("{}", x);
    let mut returnable: f64 = 0.0;

    while returnable == 0.0 {
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        match trimmed.parse::<f64>() {
            Ok(i) => return i,
            Err(..) => println!("this was not an positive integer: {}", trimmed),
        };
    }
    return 0.0;
}

fn foo(x: f64, h: f64) -> f64 {
    if x <= 0.0 {
        return 1.0;
    }

    let first = foo(x - 1.0, h);
    let second = F(first);
    let third = h * second;
    return first + third;
}

fn F(y: f64) -> f64 {
    let first: f64 = 1.0;
    let second: f64 = 2.0 * y;
    return first / second;
}
