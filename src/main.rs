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

// a function called story_teller that takes a player struct and an action enum as parameters and returns as string
fn story_teller(player: Survivor, action: Action) -> String {
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
            story = format!("You slept for {} minutes.", player.sleep);
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

    story
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

    println!("You woke up on a deserted island feeling dazed and confused.");

    while player.alive {
        clear_screen();

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
        println!("");
    }

    fn drink(player: &mut Survivor) {
        player.water += 10;
        player.hunger -= 10;
        player.sanity -= 10;
        println!("You drank some water.");
        println!("");
    }

    fn sleep(player: &mut Survivor) {
        // generate a random number between 1 and 10
        let sleep_amount = rand::thread_rng().gen_range(1..500);
        player.sleep += sleep_amount;
        player.hunger -= 10;
        player.water -= 10;
        player.sanity -= 10;
        println!("You slept for {} minutes.", sleep_amount);
        println!("");
    }

    fn clear_screen() {
        system_sleep();
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    fn system_sleep() {
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
