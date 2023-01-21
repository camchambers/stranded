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

// Survivor implementation
impl Survivor {
    fn new() -> Survivor {
        Survivor {
            health: 100,
            hunger: 100,
            water: 100,
            sleep: 100,
            sanity: 100,
            alive: true,
        }
    }

    fn eat(&mut self) {
        self.hunger += 10;
        self.water -= 10;
        print!("You ate some food.");
    }

    fn drink(&mut self) {
        self.water += 10;
        self.hunger -= 10;
        print!("You drank some water.");
    }

    fn sleep(&mut self) {
        self.sleep += 10;
        self.sanity -= 10;
        print!("You slept for a while.");
    }
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

// a function for getting the player's  input
fn get_player_action() -> String {
    println!("What would you like to do? \n");
    println!("1. Search for food.");
    println!("2. Search for water.");
    println!("3. Sleep");
    println!("");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    // create a new survivor called player
    let mut player = Survivor::new();

    while player.alive {
        clear_screen();

        display_player_stats(&mut player);

        let action = get_player_action();

        if action == "1" {
            player.eat();
        }else if action == "2"{
            player.drink();
        }else if action == "3"{
            player.sleep();
        }else {
            println!("Please enter a valid choice.");
        }
    }

    fn clear_screen() {
        system_sleep();
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    fn system_sleep() {
        std::thread::sleep(std::time::Duration::from_millis(3000));
    }
}
