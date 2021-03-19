mod test;
mod gol;

use std::collections::HashMap;
use gol::gol::{Coordinate, Cell, bring_to_life, next};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut game: HashMap<Coordinate, Cell> = HashMap::new();
    let mut game_prime: HashMap<Coordinate, Cell> = HashMap::new();

    let buddies = [
        Coordinate { x: 4, y: 2 },
        Coordinate { x: 2, y: 3 },
        Coordinate { x: 4, y: 3 },
        Coordinate { x: 3, y: 4 },
        Coordinate { x: 4, y: 4 }];

    buddies.iter().for_each(|b| bring_to_life(&mut game, &b));

    fn game_loop (g: &mut HashMap<Coordinate, Cell>, gg: &mut HashMap<Coordinate, Cell>) {
        next(g, gg);
        println!("{:?}", gg.keys());
        g.clear();
        game_loop(gg, g);
    }

    game_loop(&mut game, &mut game_prime);
}