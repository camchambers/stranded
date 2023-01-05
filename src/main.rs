/**
 * A text-based game where you try to survive on an island with limited resources. 
 */

 struct Survivor {
    health: f32,
    hunger: f32,
    water: f32,
    sleep: f32,
    sanity: f32,
    alive: bool,
}

fn main() {

    // instantiate the player struct with all values set to 100
    let mut player: Survivor = Survivor {
        health: 100.0,
        hunger: 100.0,
        water: 100.0,
        sleep: 100.0,
        sanity: 100.0,
        alive: true,
    };

    println!("You woke up on a deserted island feeling dazed and confused.");

    while player.alive {
        println!("What would you like to do?");
        println!("1. Eat food.");
        println!("2. Drink water.");
        println!("3. Sleep");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: u32 = input.trim().parse().expect("Please type a number!");
    }

}