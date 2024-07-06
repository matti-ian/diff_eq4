//Solving the differential equation: y'=x^2 sin y numerically
use std::io;
fn main() {
    println!("Solving the differential equation: y'=x^2 sin(y) numerically");
    let mut input = String::new();

    println!("Enter initial value of y:");
    io::stdin().read_line(&mut input).expect("Error reading");
    let mut y:f32 = input.trim().parse().expect("Error parsing");
    input.clear();

    println!("Enter initial value of x:");
    io::stdin().read_line(&mut input).expect("Error reading");
    let mut x:f32 = input.trim().parse().expect("Error parsing");
    input.clear();

    println!("Enter step value, dx:");
    io::stdin().read_line(&mut input).expect("Error reading");
    let  dx:f32 = input.trim().parse().expect("Error parsing");

    for _ in 0..20{
            //y + dy
        let dy = x.powi(2) * y.sin();
        y = y + dy;
        x = x + dx;
        println!("x = {:.4},\t y = {:.4}", x, y);
    }
    println!("Step value: {}",dx);
}
