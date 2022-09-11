use std::io;

fn main() {
    let mut f = String::new();

    println!("Enter a degree Fahrenheit to convert it into degree Celsius: ");

    io::stdin()
        .read_line(&mut f)
        .expect("Failed to read the line");

    let f = f.trim().parse::<i32>().expect("Failed to parse the number");

    let c = (f - 32) * 5 / 9;

    println!("{} °F = {} °C", f, c);
}
