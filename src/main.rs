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

        clear_screen();

        println!("What would you like to do?");
        println!("1. Eat food.");
        println!("2. Drink water.");
        println!("3. Sleep");


        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
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
        player.hunger += 10.0;
        player.water -= 10.0;
        player.sanity -= 10.0;
        println!("You ate some food.");
    }

    fn drink(player: &mut Survivor) {
        player.water += 10.0;
        player.hunger -= 10.0;
        player.sanity -= 10.0;
        println!("You drank some water.");
    }

    fn sleep(player: &mut Survivor) {
        player.sleep += 10.0;
        player.hunger -= 10.0;
        player.water -= 10.0;
        player.sanity -= 10.0;
        println!("You slept for a bit.");
    }

    fn clear_screen() {
        system_sleep();
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    fn system_sleep(){
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

}