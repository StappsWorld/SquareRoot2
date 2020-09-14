use std::io;

fn main() {
    println!("How precise would you like?");

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let mut n: i128 = 0;
    let trimmed = input_text.trim();
    match trimmed.parse::<i128>() {
        Ok(i) => n = i,
        Err(..) => println!("this was not an integer: {}", trimmed),
    };

    println!("Output was : {}", foo(n));
}

fn foo(mut i: i128) -> f64 {
    if i <= 0 {
        return 1.0;
    }

    let first = foo(i - 1);
    let second = F(first);
    let third = f(first);

    return first - (second / third);
}

fn F(x: f64) -> f64 {
    return (x * x) - 2.0;
}

fn f(x: f64) -> f64 {
    return 2.0 * x;
}
