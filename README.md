# sync-PRNG-Rust-NumPy
>[!NOTE]
>Find valid seeds to sync Rust Mersenne Twister PRNG from Crate "mt19937" to Python NumPy PRNG

## test_numpy_vs_rust.py
> [!IMPORTANT]
Seed must be set to the same seed used to run the rust program.
>[!NOTE]
> Used to test rust_random_vals.txt against NumPy PRNG.

## create_file_for_rust.py
> [!IMPORTANT]
Seed is set within the program.
> [!NOTE]
> Creates json file containing list of samples using NumpPy PRNG.
> Files are created in: ../data/
> Example file: "rng_vals_seed90487.json"

## main.rs
> [!IMPORTANT]
https://crates.io/crates/mersenne_twister requires rand >= 0.3, < 0.5
Seed value from CLI is also used in file path for correct json file.

> [!NOTE]
>Flags:  
>1. -m pass 0 to use: https://crates.io/crates/mt19937 (recomended)
           >pass 1 to use: https://crates.io/crates/mersenne_twister
>2. -e pass flag if you want to use == for float comparison (by default uses approx_eq! from https://crates.io/crates/float-cmp
>3. -g pass flag if you want to generate rust_random_vals.txt
>4. -s seed value for PRNG

