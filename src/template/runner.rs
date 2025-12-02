/// Encapsulates code that interacts with solution functions.
use std::fmt::Display;
use std::hint::black_box;
use std::process::Output;
use std::time::{Duration, Instant};
use std::{env, process};

use criterion::{BenchmarkId, Criterion};

use crate::template::ANSI_BOLD;
use crate::template::{ANSI_RESET, Day, aoc_cli};

pub fn run_part<I: Copy, T: Display>(
    func: impl Fn(I) -> Option<T>,
    input: I,
    day: Day,
    part: u8,
    bench: &mut Criterion,
) {
    let part_str = format!("Part {part}");
    let timer = Instant::now();
    let result = {
        #[cfg(feature = "dhat-heap")]
        let _profiler = dhat::Profiler::new_heap();

        func(input)
    };
    let base_time = timer.elapsed();
    let timed = std::env::args().any(|x| x == "--time");

    if !timed || base_time > Duration::from_secs(1) {
        print_result(&result, &part_str, &format_duration(&base_time, 1));
        return;
    }

    let mut total_runtime = Duration::ZERO;
    let mut samples = 0;
    bench.bench_with_input(
        BenchmarkId::new(format!("Day {day}"), part),
        &input,
        |b, &input| {
            b.iter_custom(|iters| {
                let start = Instant::now();
                for _ in 0..iters {
                    black_box(func(black_box(input)));
                }
                let took = start.elapsed();
                total_runtime += took;
                samples += iters as u32;
                took
            })
        },
    );


    print_result(&result, &part_str, &format_duration(&(total_runtime / samples), samples ));

    if let Some(result) = result {
        submit_result(result, day, part);
    }
}

fn format_duration(duration: &Duration, samples: u32) -> String {
    if samples == 1 {
        format!(" ({duration:.1?})")
    } else {
        format!(" ({duration:.1?} @ {samples} samples)")
    }
}

fn print_result<T: Display>(result: &Option<T>, part: &str, duration_str: &str) {
    match result {
        Some(result) => {
            if result.to_string().contains('\n') {
                let str = format!("{part}: ▼ {duration_str}");
                print!("\r");
                println!("{str}");
                println!("{result}");
            } else {
                let str = format!("{part}: {ANSI_BOLD}{result}{ANSI_RESET}{duration_str}");
                print!("\r");
                println!("{str}");
            }
        }
        None => {
            print!("\r");
            println!("{part}: ✖             ");
        }
    }
}

/// Parse the arguments passed to `solve` and try to submit one part of the solution if:
///  1. we are in `--release` mode.
///  2. aoc-cli is installed.
fn submit_result<T: Display>(
    result: T,
    day: Day,
    part: u8,
) -> Option<Result<Output, aoc_cli::AocCommandError>> {
    let args: Vec<String> = env::args().collect();

    if !args.contains(&"--submit".into()) {
        return None;
    }

    if args.len() < 3 {
        eprintln!("Unexpected command-line input. Format: cargo solve 1 --submit 1");
        process::exit(1);
    }

    let part_index = args.iter().position(|x| x == "--submit").unwrap() + 1;

    let Ok(part_submit) = args[part_index].parse::<u8>() else {
        eprintln!("Unexpected command-line input. Format: cargo solve 1 --submit 1");
        process::exit(1);
    };

    if part_submit != part {
        return None;
    }

    if aoc_cli::check().is_err() {
        eprintln!(
            "command \"aoc\" not found or not callable. Try running \"cargo install aoc-cli\" to install it."
        );
        process::exit(1);
    }

    println!("Submitting result via aoc-cli...");
    Some(aoc_cli::submit(day, part, &result.to_string()))
}
