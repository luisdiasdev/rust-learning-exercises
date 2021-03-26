use std::io;
use std::io::prelude::*;

#[derive(Debug)]
enum Classification {
    Underweight,
    Normal,
    Overweight,
    Obesity,
    Unknown
}

fn get_classification_by_bmi(bmi: f64) -> Classification {
    match bmi {
        x if x < 0.0 => Classification::Unknown,
        x if x < 18.5 => Classification::Underweight,
        x if x < 24.9 => Classification::Normal,
        x if x < 29.9 => Classification::Overweight,
        x if x >= 29.9 => Classification::Obesity,
        _ => Classification::Unknown
    }
}

fn calculate(height: f64, weight: f64) -> f64 {
    weight / height.powf(2.0)
}

fn main() {
    println!("Body Mass Index Calculator");
    println!("- Please type your height in centimeters: ");
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let height_string = iterator.next().unwrap().unwrap();
    let height = height_string.parse::<f64>().unwrap();
    println!("Height: {}", height);
    println!("- Now type your weight in kilograms: ");
    let weight_string = iterator.next().unwrap().unwrap();
    let weight = weight_string.parse::<f64>().unwrap();
    println!("Weight: {}", weight);
    let bmi = calculate(height / 100.0, weight);
    let classification = get_classification_by_bmi(bmi);
    println!("Your BMI is: {:.2}", bmi);
    println!("Your current classification is: {:?}", classification);
}
