use std::collections::{HashMap, HashSet};
use std::hash::{Hash};
use ndarray::{arr2, Array2};
use std::fmt::{Display, Formatter, Result, Debug};

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
pub struct Cell {
    pub alive: bool
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", if self.alive { "0" } else { "x" })
    }
}

pub type World = HashMap<Coordinate, Cell>;

pub struct Game {
    pub world: World
}

impl Debug for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let max_x = self.world.keys().map(|coord| coord.x).max().unwrap_or(0);
        let max_y = self.world.keys().map(|coord| coord.y).max().unwrap_or(0);

        let mut world = String::new();

        for y in 0..max_y+1 {
            for x in 0..max_x+2 {
                if self.is_alive(&Coordinate { x, y }) {
                    world.push('0');
                } else {
                    world.push('.');
                }
            }
            world.push('\n');
        }

        write!(f, "{}", world)
    }
}

const NEIGHBOURS: [[i32; 2]; 9] =
    [[-1, -1], [0, -1], [1, -1],
        [-1, 0], [0, 0], [1, 0],
        [-1, 1], [0, 1], [1, 1]];

pub fn get_neighbours(coord: &Coordinate) -> HashSet<Coordinate> {
    let neighbours: Array2<i32> = arr2(&NEIGHBOURS);

    let ones: Array2<i32> = Array2::<i32>::ones((9, 2));

    let coord_vector = arr2(
        &[[coord.x, 0],
            [0, coord.y]]);

    let cellmates: Array2<i32> = ones.dot(&coord_vector) + neighbours;

    let mut set: HashSet<Coordinate> = cellmates.outer_iter().map(|row| Coordinate { x: row[0], y: row[1] }).collect();
    set.remove(coord);
    set
}

impl Iterator for Game {
    type Item = World;

    fn next(&mut self) -> Option<Self::Item> {

        let interesting = self.get_interesting_neighbours();
        let mut future = Game{world: World::new()};

        interesting.iter().for_each(|coord|
            match self.count_living_neighbours(coord) {
                2 => if self.is_alive(coord) {
                        future.bring_to_life(coord)
                },
                3 => future.bring_to_life(coord),
                _ => ()
            });
        self.world = future.world;
        Some(self.world.clone())
    }
}

impl Game {
    pub fn is_alive(&self, coordinate: &Coordinate) -> bool {
        match self.world.get(coordinate) {
            Some(found) => found.alive,
            None => false
        }
    }

    pub fn bring_to_life(&mut self, coordinate: &Coordinate) {
        self.world.insert(*coordinate, Cell { alive: true });
    }

    pub fn kill(&mut self, coordinate: &Coordinate) {
        self.world.remove(coordinate);
    }

    pub fn count_living_neighbours(&self, coord: &Coordinate) -> usize {
        get_neighbours(coord).iter().filter(|c| self.is_alive(c)).count()
    }

    pub fn get_interesting_neighbours(&self ) -> HashSet<Coordinate> {
        let neighbour_sets = self.world.iter().map(|(coord, _)| get_neighbours(coord));
        let mut interesting = HashSet::new();
        neighbour_sets.for_each(
            |interesting_neighbour_set| {
                interesting.extend(interesting_neighbour_set.iter().filter(|c| c.x >= 0 && c.y >= 0));
            });
        interesting
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashSet};
    use crate::gol::*;
    use crate::parse::parse_world;

    #[test]
    fn a_cell_comes_to_life() {
        let mut game = Game { world: World::new() };

        let home = Coordinate { x: 0, y: 0 };

        assert_eq!(false, game.is_alive(&home));
        game.bring_to_life( &home);

        assert_eq!(true, game.is_alive( &home));
    }

    #[test]
    fn a_cell_can_die() {
        let mut game = Game { world: World::new() };
        let home = Coordinate { x: 0, y: 0 };

        game.bring_to_life(&home);
        game.kill(&home);
        assert_eq!(false, game.is_alive( &home));
    }

    #[test]
    fn a_living_cell_with_one_neighbours_dies_from_loniness() {
        let mut game = parse_world(".00.");

        let subject = Coordinate { x: 1, y: 0};

        assert_eq!(true, game.is_alive( &subject));

        game.next();

        assert_eq!(false, game.is_alive(&subject));
    }

    #[test]
    fn a_dead_cell_with_three_neighbours_comes_to_life() {
        let mut game = parse_world("..0..\n\
                                                 .0.0.\n\
                                                 .....");

        let subject = Coordinate { x: 2, y: 1};

        assert_eq!(false, game.is_alive( &subject));

        game.next();

        assert_eq!(true, game.is_alive(&subject));
    }

    #[test]
    fn a_living_cell_with_two_neighbours_stays_live() {
        let mut game = parse_world(
            "..0..\n\
                   ..0..\n\
                   ..0..");

        let subject = Coordinate { x: 2, y: 1};

        assert_eq!(true, game.is_alive( &subject));
        assert_eq!(3, game.count_living_neighbours(& Coordinate {x: 3, y: 1}));
        game.next();

        assert_eq!(true, game.is_alive(&subject));
        assert_eq!(true, game.is_alive(&Coordinate {x: 3, y: 1}));
    }

    #[test]
    fn a_living_cell_with_four_or_more_neighbours_dies_from_over_crowding() {
        let mut game = parse_world("..0..\n\
                                                 .000.\n\
                                                 ..0..");

        let subject = Coordinate { x: 2, y: 1};

        assert_eq!(true, game.is_alive( &subject));

        game.next();

        assert_eq!(false, game.is_alive(&subject));
    }


    #[test]
    fn a_dead_cell_with_two_neighbours_stays_dead() {
        let mut game = parse_world(".....\n\
                                              .0.0.\n\
                                              .....");

        let subject = Coordinate { x: 2, y: 1};

        assert_eq!(false, game.is_alive( &subject));

        game.next();

        assert_eq!(false, game.is_alive(&subject));
    }

    #[test]
    fn should_list_neighbours_of_a_coordinate() {
        let coord = Coordinate {
            x: 2,
            y: 3,
        };

        let neighbours = get_neighbours(&coord);

        assert_eq!(true, neighbours.contains(&Coordinate { x: 3, y: 4 }));
        assert_eq!(true, neighbours.contains(&Coordinate { x: 1, y: 2 }));
    }

    #[test]
    fn should_compile_all_interesting_coordinates() {
        let mut game = Game{ world: World::new()};

        parse_world("0.0");
        game.bring_to_life( &Coordinate { x: 0, y: 0 });
        game.bring_to_life( &Coordinate { x: 2, y: 0 });

        let mut expected: HashSet<Coordinate> = HashSet::new();

        expected.insert(Coordinate { x: 1, y: 0 });
        expected.insert(Coordinate { x: 3, y: 0 });
        expected.insert(Coordinate { x: 1, y: 1 });
        expected.insert(Coordinate { x: 0, y: 1 });
        expected.insert(Coordinate { x: 2, y: 1 });
        expected.insert(Coordinate { x: 3, y: 1 });


        let interesting = game.get_interesting_neighbours();

        assert_eq!(expected, interesting);
    }

    #[test]
    fn should_count_living_neighbours() {

        let game = parse_world(
            "00..\n\
                   0000");

        assert_eq!(3,game.count_living_neighbours(&Coordinate{x: 0, y: 0}));
        assert_eq!(4, game.count_living_neighbours(&Coordinate { x: 1, y: 1 }));
        assert_eq!(1, game.count_living_neighbours(&Coordinate{x:3, y: 1}));
        assert_eq!(3, game.count_living_neighbours(&Coordinate{x: 2, y: 1}));
    }
}