use std::io;

fn main() {
    let mut input = String::new();
    println!("Please enter the unit (C/F)");
    io::stdin().read_line(&mut input).expect("Not able to read the unit!");
    let unit = input.trim();
    if unit == "F" {
        input.clear();
        println!("Please enter the temperature in Farenheit unit.");
        io::stdin().read_line(&mut input).expect("Not able to read the temperature!");
        let temp:f64 = input.trim().parse().expect("Not able to parse temp input");
        println!("Input temp in Farenheit: {}, converted temp in Celcius: {}", temp, convert_to_celcius(temp));
    } else if unit == "C" {
        input.clear();
        println!("Please enter the temperature in Celcius unit.");
        io::stdin().read_line(&mut input).expect("Not able to read the temperature!");
        let temp:f64 = input.trim().parse().expect("Not able to parse temp input");
        println!("Input temp in Celcius: {}, converted temp in Farenheit: {}", temp, convert_to_farenheit(temp));
    } else {
        println!("Unit should be either \"C\" or \"F\"");
    }



}

// convert value from Farenheit to Celcius
fn convert_to_celcius(temp :f64) -> f64 {
    ((temp - 32.0) * 5.0)/9.0
}

// convert from Celcius to Farenheit
fn convert_to_farenheit(temp: f64) -> f64 {
    ((temp / 5.0) * 9.0) + 32.0
}
