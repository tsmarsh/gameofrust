use std::collections::HashMap;
use crate::gol::{Coordinate, Cell, World, Game};
use std::str::Chars;

fn parse_char(world_string: &mut Chars, x: i32, y: i32, game: &mut HashMap<Coordinate, Cell>) {
    if let Some(s) = world_string.next() {
        match s {
            '.' => parse_char(world_string, x + 1, y, game),
            '0' => {
                game.insert(Coordinate { x, y }, Cell { alive: true });
                parse_char(world_string, x + 1, y, game)
            }
            '\n' => parse_char(world_string, 0, y + 1, game),
            _ => unimplemented!()
        }
    }
}

pub fn parse_world(world: &str) -> Game{
    let mut game = World::new();
    parse_char(&mut world.chars(), 0, 0, &mut game);
    Game{world: game}
}

#[cfg(test)]
mod test {
    use crate::parse::{parse_world};
    use crate::gol::{Coordinate};

    #[test]
    fn can_parse_a_world() {
        let world = "...\n\
                           .00\n\
                           ...";

        let game = parse_world(world);

        assert_eq!(true, game.is_alive( &Coordinate { x: 2, y: 1 }));
        assert_eq!(true, game.is_alive( &Coordinate { x: 1, y: 1 }));
        assert_eq!(false, game.is_alive( &Coordinate { x: 0, y: 0 }));
    }
}