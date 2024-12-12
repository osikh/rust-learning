fn main() {
    println!("Hello, world!");

    mars();
    send_aliens_on_earth(10);

    let tnt_required = calc_tnt(8);
}

fn mars() {
    println!("Hello I'm calling from Mars!")
}

fn send_aliens_on_earth(number_of_aliens: usize) {
    println!("Sending {} aliens to attack Earth 🫡", number_of_aliens)
}

fn calc_tnt(population_in_billions: usize) -> f64 {
    println!("Total population on Earth: {}", population_in_billions);

    let tnt_required_to_kill_1_human = 50; // in kg
    println!("TNT (kg) required to kill 1 human in kg: {}", tnt_required_to_kill_1_human);

    let error_calc = 1.5; // for uneven ground and bunker area

    let tnt_required: usize = population_in_billions.pow(9) * tnt_required_to_kill_1_human;
    let tnt_required = error_calc * (tnt_required as f64);
    let tnt_required_in_ton = tnt_required / 1000.0;

    println!("TNT (ton) required to annihilate human race: {}", tnt_required_in_ton);

    return tnt_required;
}