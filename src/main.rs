fn main() {
    println!("Hello, world!");
}
mod gol {
    use std::collections::HashMap;
    use std::hash::{Hash};

    #[derive(Hash, PartialEq, Eq, Copy, Clone)]
    pub struct Coordinate {
        pub x: u32,
        pub y: u32
    }

    #[derive(Hash, PartialEq, Eq)]
    pub struct Cell {
        pub alive: bool
    }

    pub fn is_alive(game: &HashMap<Coordinate, Cell> , coordinate: &Coordinate) -> bool{
        match game.get(coordinate) {
            Some(found) => found.alive,
            None => false
        }
    }

    pub fn bring_to_life(game: &mut HashMap<Coordinate, Cell> , coordinate: &Coordinate) {
        game.insert(*coordinate, Cell{alive: true});
    }

    pub fn kill(game: &mut HashMap<Coordinate, Cell>, coordinate: &Coordinate) {
        game.remove(coordinate);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::gol::*;

    #[test]
    fn a_cell_comes_to_life() {
        let mut game:HashMap<Coordinate, Cell> = HashMap::new();
        let home = Coordinate{x:0, y:0};

        assert_eq!(false, is_alive(&game, &home));
        bring_to_life(&mut game, &home);

        assert_eq!(true, is_alive(&game, &home));
    }

    #[test]
    fn a_cell_can_die(){
        let mut game:HashMap<Coordinate, Cell> = HashMap::new();
        let home = Coordinate{x:0, y:0};

        bring_to_life(&mut game, &home);
        kill(&mut game, &home);
        assert_eq!(false, is_alive(&game, &home));
    }
}