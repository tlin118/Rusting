use std::io;


fn f_to_c() {
    println!("Enter the temperature");

    let mut f_temp = String::new();

    io::stdin()
        .read_line(&mut f_temp)
        .expect("Failed to read line");

    let f_temp: f32 = f_temp.trim().parse().expect("Not a number");

    let f_converted = (f_temp - 32.0) * (5.0 / 9.0);

    println!("Temperature in Celsius is: {f_converted}");
}

fn c_to_f() {
    println!("Enter the temperature");

    let mut c_temp = String::new();

    io::stdin()
        .read_line(&mut c_temp)
        .expect("Failed to read line");

    let c_temp: f32 = c_temp.trim().parse().expect("Not a number");

    let c_converted = c_temp * (9.0 / 5.0) + 32.0;

    println!("Temperature in Fahrenheit is: {c_converted}");
}



fn main() {
    println!("Select the conversion type");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");
    
    let mut selection = String::new();

    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");

    let selection: i32 = selection.trim().parse().expect("Not a number");

    if selection != 1 && selection != 2 {
        println!("Did not select the correct option");
    } else if selection == 1 {
        println!("You have chosen Fahrenheit to Celsius conversion");
        f_to_c();
    } else {
        println!("You have chosen Celsius to Fahrenheit conversion");
        c_to_f();
    }

}
