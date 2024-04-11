use core::panic;
use float_cmp::approx_eq;
use std::fs::{self, File};

use clap::{arg, command, Parser};
use rand::Rng;
use std::io::{BufWriter, Write};

// use mersenne_twister::MersenneTwister;
// Short program to generate 1000 numbers with range 0.0 1.0 using mersenne twister
// Important note, only some seeds will work as to replicate with np.random we need to dissect and rebuild state.

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)] //, value_name = "mt version, 0 mt19937 1 mersenne twister MT19937 64")]
    mt_version: usize,
    #[arg(short, long, value_name = "use == comparisson or assert")]
    exact_comparison: bool,
    #[arg(
        short,
        long,
        value_name = "true generate file of rng vals false: compare to seed file"
    )]
    generate_file: bool,
    #[arg(short, long, value_name = "seed value")]
    seed: u32,
}

fn write_rand_vals(args: &Args, vals: &[f64]) {
    let data_file = File::create(shellexpand::full("~/testing_mersenne/data/rust_random_vals.txt").unwrap().as_ref()).expect("Cannot write file");
    let mut file = BufWriter::new(data_file);
    write!(file, "rust =[ "); //if no space after [ first digit gets cut off when we write array

    let (arr, _) = gen_rand_vals(args, vals, false);

    arr.iter()
        .for_each(|v| write!(file, "{},", v).expect("Cannot write to file."));
    write!(file, "]");
}

// returns a vector of all values generated and a vector contianing off by x errors
fn gen_rand_vals(args: &Args, vals: &[f64], output: bool) -> (Vec<f64>, Vec<f64>) {
    let mut mers_rng0 = mt19937::MT19937::new_with_slice_seed(&[args.seed]);

    let mut mers_rng1: mersenne_twister::MT19937_64 = rand::SeedableRng::from_seed(args.seed as u64);
    let mut failures_off_by: Vec<f64> = Vec::new();
    let mut gen_values: Vec<f64> = Vec::new();

    for py_val in vals.iter() {
        //Ignore warn needed if we want to write out
        let gen: f64 = if args.mt_version == 0 {
            // Usual method of generation doesnt work
            mt19937::gen_res53(&mut mers_rng0)
        } else if args.mt_version == 1 {
            mers_rng1.gen_range(0.0, 1.0)
        } else {
            panic!("Invalid value for args.mt_version");
        };

        gen_values.push(gen);
        if output {
            if if args.exact_comparison {
                gen != *py_val
            } else {
                !approx_eq!(f64, gen, *py_val)
            } {
                print!("FAILED gen:{} py:{} __ ", gen, py_val);
                println!("Off by: {}", py_val - gen);
                failures_off_by.push(py_val - gen);
            } else if if args.exact_comparison {
                gen == *py_val
            } else {
                approx_eq!(f64, gen, *py_val)
            } {
                println!("Exact match gen:{} py:{}", gen, py_val);
            }
        }
    }
    (gen_values, failures_off_by)
}

fn rand_vals(args: &Args, vals: &[f64]) {
    let (_, failures_off_by) = gen_rand_vals(args, vals, true);
    println!("Total errors: {} out of", failures_off_by.len());

    match failures_off_by.is_empty() {
        true => {
            println!("all values generated exactly match.");
        }
        false => {
            println!(
                "Average error: {} out of: {}",
                failures_off_by.iter().sum::<f64>() / failures_off_by.len() as f64,
                vals.len()
            );
            println!(
                "Max error: {}",
                failures_off_by
                    .iter()
                    .max_by(|a, b| a.total_cmp(b))
                    .unwrap()
            );
            println!(
                "Min error: {}",
                failures_off_by
                    .iter()
                    .max_by(|b, a| a.total_cmp(b))
                    .unwrap()
            );
        }
    }
}
fn main() {
    let args: Args = Args::parse();

    // let seed: u32 = 91298;
    let _seeds: Vec<u32> = parse_json("~/testing_mersenne/data/valid_seed.txt");
    let path: String = "~/testing_mersenne/data/rng_vals_seed".to_owned()
        + args.seed.to_string().as_str()
        + ".json";
    let vals: Vec<f64> = parse_json(&path);

    println!("Testing mersenne twister in rust for 10000 samples");
    println!("{}", args.exact_comparison);
    // If we want to write rust random generation to file
    if args.generate_file {
        write_rand_vals(&args, &vals);
    } else {
        rand_vals(&args, &vals);
    }
}

pub fn parse_json<T: for<'a> serde::Deserialize<'a>>(path: &str) -> T {
    let data =
        fs::read_to_string(shellexpand::full(&path).unwrap().as_ref()).unwrap_or_else(|_| {
            panic!(
                "{}",
                ("Unable to read file path provided: ".to_owned() + path)
            )
        });

    serde_json::from_str(&data)
        .unwrap_or_else(|_| panic!("{}", ("JSON was not well-formatted: ".to_owned() + path)))
}
