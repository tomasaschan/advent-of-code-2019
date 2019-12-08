pub fn solve_a(input: &String) -> i32 {
    input
        .split_whitespace()
        .map(|x| -> i32 { x.parse().expect("invalid input") })
        .map(fuel_for_mass)
        .sum()
}
pub fn solve_b(input: &String) -> i32 {
    input
        .split_whitespace()
        .map(|x| -> i32 { x.parse().expect("invalid input") })
        .map(rocket_eq_solution)
        .sum()
}
fn fuel_for_mass(w: i32) -> i32 {
    if w / 3 - 2 > 0 {
        w / 3 - 2
    } else {
        0
    }
}
fn rocket_eq_solution(w: i32) -> i32 {
    let fuel = fuel_for_mass(w);
    if fuel > 0 {
        fuel + rocket_eq_solution(fuel)
    } else {
        fuel
    }
}
