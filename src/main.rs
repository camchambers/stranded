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

// an enum of player actions that can be taken
enum Action {
    Start,
    Eat,
    Drink,
    Sleep,
    Fight,
    Flee,
    Swim,
    Die,
}

enum Actors {
    None,
    Bear,
    Scorpion,
}

// A function for displaying the current state of the player
fn display_player_stats(player: &mut Survivor) {
    println!("Health: {}", player.health);
    println!("Hunger: {}", player.hunger);
    println!("Water: {}", player.water);
    println!("Sleep: {}", player.sleep);
    println!("Sanity: {}", player.sanity);
    println!("");
}

// a function called story_teller that takes a player object and an action enum as parameters and returns as string
fn story_teller(player: &mut Survivor, action: Action, target: Actors, value: i32) {
    let mut story = String::new();

    match action {
        Action::Start => {
            story = format!("You woke up on a deserted island feeling dazed and confused.");
        }
        Action::Eat => {
            story = format!("You ate some food.");
        }
        Action::Drink => {
            story = format!("You drank some water.");
        }
        Action::Sleep => {
            if player.sleep > 60 {
                story = format!("You slept for {} minutes. You clearly over slept.", value);
            } else {
                story = format!("You slept for {} minutes You feel well rested.", value);
            }
        }
        Action::Fight => {
            story = format!("You fought a wild animal.");
        }
        Action::Flee => {
            story = format!("You fled from a wild animal.");
        }
        Action::Swim => {
            story = format!("You swam in the ocean.");
        }
        Action::Die => {
            story = format!("You died.");
        }
    }
    println!("{}", story);
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

    story_teller(&mut player, Action::Start, Actors::None, 0);


    while player.alive {
        clear_screen();

        display_player_stats(&mut player);

        println!("What would you like to do?");
        println!("1. Eat food.");
        println!("2. Drink water.");
        println!("3. Sleep");

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
        let sleep_amount = rand::thread_rng().gen_range(1..500);
        player.sleep += sleep_amount;
        player.hunger -= 10;
        player.water -= 10;
        player.sanity -= 10;
        story_teller(player, Action::Sleep, Actors::None, sleep_amount);
    }

    fn clear_screen() {
        system_sleep();
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    fn system_sleep() {
        std::thread::sleep(std::time::Duration::from_millis(3000));
    }
}
