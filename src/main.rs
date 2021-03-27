use gameofrust::parse::{parse_world};
use std::io;

fn main() {
    let mut world = parse_world(
                "......\n\
                       ...0..\n\
                       .0.0..\n\
                       ..00..");

    let mut input = String::new();

    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                println!("{:?}", world);
                world.next();
            }
            Err(error) => println!("error: {}", error),
        }
    }
}