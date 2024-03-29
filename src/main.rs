extern crate clap;
extern crate ethereum_types;
extern crate hex;
extern crate rand;
extern crate rayon;

use clap::{App, Arg};
use ethereum_types::{Address, H160};
use rand::Rng;
use rayon::prelude::*;

fn main() {
    let matches = App::new("Ethereum Vanity Address Generator")
        .version("1.0")
        .author("Your Name")
        .about("Generates Ethereum vanity addresses")
        .arg(
            Arg::with_name("prefix")
                .short('p')
                .long("prefix")
                .value_name("PREFIX")
                .help("Sets the prefix for the vanity address")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let prefix = matches.value_of("prefix").unwrap();
    let prefix_bytes = hex::decode(&prefix[2..]).expect("Invalid hex prefix");

    let address = (0..num_cpus::get())
        .into_par_iter()
        .find_any(|_| {
            let address = Address::random();
            address.as_bytes().starts_with(&prefix_bytes)
        })
        .unwrap();

    println!("Found address: 0x{}", hex::encode(address.to_be_bytes()));
}
