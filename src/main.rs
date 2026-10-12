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

fn main() {
    println!("Hello, world!");
}
