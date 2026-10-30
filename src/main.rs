use std::fs::read_to_string;

#[allow(dead_code)]
mod my_lib;

#[allow(unreachable_code)]
#[cfg(test)]
mod tests;

type Answer = u64;

fn main() {
    let args = my_lib::Args::get();
    let content = args
        .path
        .map(|p| read_to_string(&p).expect("input: Could not open file"));

    if args.part1 {
        let now = std::time::Instant::now();
        println!(
            "Part 1: {}, found in {}ms",
            part1(content.as_deref().expect("No input file was opened")),
            now.elapsed().as_micros() as f32 / 1000.0
        );
    }

    if args.part2 {
        let now = std::time::Instant::now();
        println!(
            "Part 2: {}, found in {}ms",
            part2(content.as_deref().expect("No input file was opened")),
            now.elapsed().as_micros() as f32 / 1000.0
        );
    }
}

fn part1(_input: &str) -> Answer {
    _input
        .split(',')
        .map(|outpost| {
            let pinecones = outpost.trim().parse::<Answer>().unwrap();
            ((pinecones as f64).sqrt().ceil() as u64).pow(2) - pinecones
        })
        .sum()
}

fn part2(_input: &str) -> Answer {
    todo!()
}
