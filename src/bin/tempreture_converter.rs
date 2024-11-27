use std::io;

fn main() {
    let mut temp_type = String::new();

    println!("Celsius or Fahrenheit? c/f");

    io::stdin()
        .read_line(&mut temp_type)
        .expect("Failed to read line.");

    let temp_type = temp_type.trim();

    if temp_type == "f" {
        calculate_f_to_c();
    } else if temp_type == "c" {
        calculate_c_to_f();
    } else {
        println!("Please choose a valid type.")
    }
}

fn calculate_f_to_c() {
    let mut temp = String::new();

    println!("Enter tempreture:");

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line.");

    let temp: f64 = temp.trim().parse().expect("Please enter a valid number,");

    // conversion
    let celsius = (temp - 32.0) * 5.0 / 9.0;

    println!("Temprature in Celsius is {}", celsius);
}

fn calculate_c_to_f() {
    let mut temp = String::new();

    println!("Enter tempreture:");

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line.");

    let temp: f64 = temp.trim().parse().expect("Please enter a valid number,");

    // conversion
    let fahrenheit = (temp * 9.0 / 5.0) + 3.02;

    println!("Temprature in Fahrenheit is {}", fahrenheit);
}
