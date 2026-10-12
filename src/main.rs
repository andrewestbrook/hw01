use std::io::{self, Write};

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

const START_GOLD: u64 = 100;
const FOOD_PRICE: u64 = 5;
const MAX_SEED: u64 = 9999;

const DIE_SIDES: u64 = 6;
const TRADE_DICE: u64 = 3;
const TRADE_MULTIPLIER: u64 = 2;

const HUNT_FOOD_POOR: u64 = 0;
const HUNT_FOOD_FAIR: u64 = 10;
const HUNT_FOOD_GOOD: u64 = 20;
const HUNT_FOOD_GREAT: u64 = 40;

const MENU_BUY: u64 = 1;
const MENU_HUNT: u64 = 2;
const MENU_TRADE: u64 = 3;
const MENU_STATUS: u64 = 4;
const MENU_QUIT: u64 = 5;

/// Print the five menu options.
fn print_menu() {
    println!();
    println!("--- Trading Post ---");
    println!("{MENU_BUY}. Buy food ({FOOD_PRICE} gold per unit)");
    println!("{MENU_HUNT}. Hunt");
    println!("{MENU_TRADE}. Trade with caravan");
    println!("{MENU_STATUS}. View status");
    println!("{MENU_QUIT}. Quit");
}

/// Return true if there is enough gold to pay the cost.
fn can_afford(gold: u64, cost: u64) -> bool {
    gold >= cost
}

/// Return the food gained for a hunting roll.
fn hunt_food(roll: u64) -> u64 {
    if roll <= 1 {
        HUNT_FOOD_POOR
    } else if roll <= 3 {
        HUNT_FOOD_FAIR
    } else if roll <= 5 {
        HUNT_FOOD_GOOD
    } else {
        HUNT_FOOD_GREAT
    }
}

/// Ask until the user types a whole number from min to max (inclusive).
fn read_number(min: u64, max: u64) -> u64 {
    loop {
        print!("Enter a number from {min} to {max}: ");
        io::stdout().flush().expect("failed to flush stdout");

        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => {
                println!();
                println!("No more input. Goodbye!");
                std::process::exit(0);
            }
            Ok(_) => {}
            Err(_) => {
                println!("Could not read input. Try again.");
                continue;
            }
        }

        match line.trim().parse::<u64>() {
            Ok(value) if value >= min && value <= max => return value,
            Ok(_) => println!("That number is out of range."),
            Err(_) => println!("Please enter a whole number."),
        }
    }
}

/// Roll one six-sided die using a generator seeded from seed.
fn roll_die(seed: u64) -> u64 {
    let mut rng = StdRng::seed_from_u64(seed);
    rng.random_range(1..=DIE_SIDES)
}

/// Roll count dice with seeds seed, seed + 1, ... and return their sum.
fn roll_many(seed: u64, count: u64) -> u64 {
    let mut total = 0;
    for offset in 0..count {
        total += roll_die(seed + offset);
    }
    total
}

/// Print the player's gold, food, and day.
fn print_status(gold: u64, food: u64, day: u64) {
    println!();
    println!("--- Status ---");
    println!("Day:  {day}");
    println!("Gold: {gold}");
    println!("Food: {food}");
}

/// Hold the game state, run the menu loop, and call the other functions.
fn main() {
    println!("Welcome to Trail Trader!");
    println!("Choose a seed for this journey.");
    let seed = read_number(1, MAX_SEED);

    let mut gold = START_GOLD;
    let mut food = 0;
    let mut day = 1;
    let mut roll_count = 0;

    loop {
        print_menu();
        let choice = read_number(MENU_BUY, MENU_QUIT);

        if choice == MENU_BUY {
            if !can_afford(gold, FOOD_PRICE) {
                println!("You cannot afford any food.");
                continue;
            }
            let max_units = gold / FOOD_PRICE;
            println!("How many units of food? (max {max_units})");
            let units = read_number(1, max_units);
            gold -= units * FOOD_PRICE;
            food += units;
            day += 1;
            println!("You bought {units} food for {} gold.", units * FOOD_PRICE);
        } else if choice == MENU_HUNT {
            let roll = roll_die(seed + roll_count);
            roll_count += 1;
            let gained = hunt_food(roll);
            food += gained;
            day += 1;
            println!("You rolled a {roll} and gained {gained} food.");
        } else if choice == MENU_TRADE {
            let total = roll_many(seed + roll_count, TRADE_DICE);
            roll_count += TRADE_DICE;
            let earned = TRADE_MULTIPLIER * total;
            gold += earned;
            day += 1;
            println!("The caravan dice totaled {total}. You earned {earned} gold.");
        } else if choice == MENU_STATUS {
            print_status(gold, food, day);
        } else {
            println!("Final status:");
            print_status(gold, food, day);
            println!("Safe travels!");
            break;
        }
    }
}
