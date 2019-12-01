fn main() {
    let input = include_str!("2019-1.in");
    let fuel = input.lines().fold(0.0, |a, l| {
        let mass: f64 = l.parse().unwrap();
        let mut fuel = calculate_fuel(mass);
        let mut new_fuel = calculate_fuel(fuel);
        while new_fuel > 0.0 {
            fuel += new_fuel;
            new_fuel = calculate_fuel(new_fuel);
        }

        a + fuel
    });

    println!("{}", fuel);
}

fn calculate_fuel(mass: f64) -> f64 {
    (mass / 3.0).floor() - 2.0
}
