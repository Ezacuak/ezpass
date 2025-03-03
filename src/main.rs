use rand::prelude::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about="Simple password generator", long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 12)]
    length: u32,
}

fn main() {

    let args = Args::parse();

    gen_password(args.length);
}

fn gen_password(l: u32) {
    let mut rng = rand::rng();
    let mut psswrd = String::new();

    for _ in 0..l {
        let x: u8 = rng.random_range(33..126);
        psswrd.push(x as char);

    }

    println!("{}", psswrd);

}