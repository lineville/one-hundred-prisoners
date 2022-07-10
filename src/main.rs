// * 100 Prisoner's Dilemma

// There are 100 prisoners who are all given a random number between 0 and 100.

// There are 100 boxes with random numbers between 0 and 100, inside of them.

// Each prisoner gets to go into the room with the boxes and open up half of them,
// if all of the prisoners find their number, they all survive, if any of them fail,
// they all die.

// This will simulate the prisoners dilemma problem using different box selection strategies
// and see how the prisoners fared.

// * Naive strategy: Pick 50 boxes at random

// * Loop strategy: Pick a box, then chose the box with that number in it,
// keep following this pattern until the loop is closed, meaning we got back to our number.
// Worst case: There is a single loop of  51+ length, which prevents any prisoner from surviving.
// Best case: Biggest loop is < 50, so everyone finds their number.

use clap::Parser;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

// Executes prisoner dilemma simulation iterations many times with prisoners and the given strategy
// Returns a f32 from 0 to 1 representing the percentage of prisoners who survived.
fn simulate_prisoner_dilemma(
    prisoners: usize,
    iterations: u32,
    strategy: fn(&Vec<usize>, usize, Option<usize>) -> usize,
) -> f32 {
    // Iterations -> Vec<bool> representing successful/failed attempts
    let simulation_results = (0..iterations).map(|_| {
        // Shuffle boxes
        let mut boxes = (0..prisoners).collect::<Vec<_>>();
        boxes.shuffle(&mut thread_rng());

        // Return result of applying strategy
        apply_strategy(boxes, prisoners, strategy)
    });

    // Percentage of successful attempts 0.0 to 1.0
    simulation_results.into_iter().filter(|s| *s).count() as f32 / iterations as f32
}

// Apply strategy to the given boxes and prisoners and return whether the strategy succeeded.
fn apply_strategy(
    boxes: Vec<usize>,
    prisoners: usize,
    strategy: fn(&Vec<usize>, usize, Option<usize>) -> usize,
) -> bool {
    // Map Prisoner -> Success/Failed attempt using strategy
    let success_count = (0..prisoners)
        .map(|p| {
            let mut opened_boxes = 0;
            let mut box_contents = strategy(&boxes, p, None);

            while box_contents != p && opened_boxes < prisoners / 2 {
                opened_boxes += 1;
                box_contents = strategy(&boxes, p, Some(box_contents));
            }

            box_contents == p
        })
        .filter(|s| *s)
        .count();

    // Only succeeded if every prisoner succeeded
    success_count == prisoners
}

// Loop strategy: Pick a box, then chose the box with that number in it
fn loop_strategy(boxes: &Vec<usize>, prisoner: usize, previous: Option<usize>) -> usize {
    match previous {
        Some(previous) => boxes[previous],
        None => boxes[prisoner],
    }
}

// Naive strategy: Pick any box at random
fn _naive_strategy(boxes: &Vec<usize>, _prisoner: usize, _previous: Option<usize>) -> usize {
    boxes[rand::thread_rng().gen_range(0..boxes.len())]
}

// CLI Arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser, default_value_t = 100)]
    prisoners: u32,

    /// Number of times to greet
    #[clap(short, long, value_parser, default_value_t = 1000)]
    iterations: u32,
}

// Main function entry point
// Example usage
// > cargo run -- --prisoners 100 --iterations 1000
fn main() {
    let args = Args::parse();

    let prisoners = args.prisoners as usize;
    let iterations = args.iterations;

    println!(
        "Prisoner's dilemma {} prisoners, {} times",
        prisoners, iterations
    );

    let success_rate = simulate_prisoner_dilemma(prisoners, iterations, loop_strategy);
    println!("Prisoners: {}, Trials: {}", prisoners, iterations);
    println!("Success rate: {}%", success_rate * 100.0);
}
