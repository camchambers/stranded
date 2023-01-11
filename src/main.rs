/**
 * A text-based game where you try to survive on an island with limited resources.
 */
use rand::Rng;

struct Survivor {
    health: i32,
    hunger: i32,
    water: i32,
    sleep: i32,
    sanity: i32,
    alive: bool,
}

// A function for displaying the current state of the player
fn display_player_stats(player: &mut Survivor) {
    println!("");
    println!(
        "\t 🤎 {health}   🍲 {hunger}   💧 {water}   🛌 {sleep}   🧠 {sanity}",
        health = player.health,
        hunger = player.hunger,
        water = player.water,
        sleep = player.sleep,
        sanity = player.sanity
    );
    println!("");
}

fn main() {
    // instantiate the player struct with all values set to 100
    let mut player: Survivor = Survivor {
        health: 100,
        hunger: 100,
        water: 100,
        sleep: 100,
        sanity: 100,
        alive: true,
    };

    while player.alive {
        clear_screen();

        display_player_stats(&mut player);

        println!("What would you like to do? \n");
        println!("1. Search for food.");
        println!("2. Search for water.");
        println!("3. Sleep");

        println!("");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: u32 = input.trim().parse().expect("Please type a number!");

        if input == 1 {
            eat(&mut player);
        } else if input == 2 {
            drink(&mut player);
        } else if input == 3 {
            sleep(&mut player);
        } else {
            println!("Please enter a number between 1 and 10");
        }
    }

    fn eat(player: &mut Survivor) {
        player.hunger += 10;
        player.water -= 10;
        player.sanity -= 10;
        println!("You ate some food.");
    }

    fn drink(player: &mut Survivor) {
        player.water += 10;
        player.hunger -= 10;
        player.sanity -= 10;
        println!("You drank some water.");
    }

    fn sleep(player: &mut Survivor) {

        // generate a random number between 1 and 10
        let sleep_amount = rand::thread_rng().gen_range(0..12);
        player.sleep += sleep_amount;
        player.hunger -= (sleep_amount * 2) - rand::thread_rng().gen_range(0..5);
        player.water -= (sleep_amount * 2) - rand::thread_rng().gen_range(0..5);
        player.sanity += rand::thread_rng().gen_range(0..5);
        println!("You slept for {} hour(s).", sleep_amount);
    }

    fn clear_screen() {
        system_sleep();
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    fn system_sleep() {
        std::thread::sleep(std::time::Duration::from_millis(3000));
    }
}
