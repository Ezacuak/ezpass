use rand::{seq::IndexedRandom, Rng};
use clap::{value_parser, Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("ezpass")
        .author("Ezacuak")
        .version("1.0")
        .about("A Simple CLI to generate a password")
        .arg(
            Arg::new("length")
                .short('l')
                .long("length")
                .help("Length of the password")
                .value_parser(value_parser!(u32))
                .default_value("12")
        )
        .arg(
            Arg::new("special")
                .short('s')
                .long("special")
                .help("Enable special character => !@#$^&*")
                .action(ArgAction::SetTrue)
        )
//        .arg(
//            Arg::new("minuscule")
//                .short('m')
//                .long("minuscule")
//                .help("Enable lowercase")
//                .action(ArgAction::SetTrue)
//        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("Enable number")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("capitale")
                .short('c')
                .long("capitale")
                .help("Enable uppercase")
                .action(ArgAction::SetTrue)
        )
        .get_matches();

    let length = *matches.get_one::<u32>("length").unwrap();
    //let min = matches.get_flag("minuscule");
    let maj= matches.get_flag("capitale");
    let num= matches.get_flag("number");
    let spe = matches.get_flag("special");

    let password = gen_password(length, maj, num, spe);

    println!("{}", password);
}

fn gen_password(length: u32, maj: bool, num: bool, spe: bool) -> String {
    let mut rng = rand::rng();
    let mut password = String::new();
    let special_chars = b"!@#$%^&*()_+-=[]{}|;:,.<>?/";

    let mut choices = Vec::new();
    choices.push(1);

    if maj { choices.push(0); }
    if num { choices.push(2); }
    if spe { choices.push(3); }

    for _ in 0..length {
        let c = *choices.choose(&mut rng).unwrap();

        let x: u8 = match c {
            0 => rng.random_range(65..=90),
            1 => rng.random_range(97..=122),
            2 => rng.random_range(48..=57),
            3 => *special_chars.choose(&mut rng).unwrap(),
            _ => 0 
        };

        password.push(x as char);
    }

    password
}
