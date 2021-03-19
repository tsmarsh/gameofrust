pub mod gol {
    use std::collections::{HashMap, HashSet};
    use std::hash::{Hash};
    use ndarray::{arr2, Array2};
    use std::fmt::{Display, Formatter, Result};

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

    #[derive(Hash, PartialEq, Eq, Copy, Clone)]
    pub struct Cell {
        pub alive: bool
    }

    pub fn is_alive(game: &HashMap<Coordinate, Cell>, coordinate: &Coordinate) -> bool {
        match game.get(coordinate) {
            Some(found) => found.alive,
            None => false
        }
    }

    pub fn get_neighbours(coord: &Coordinate) -> HashSet<Coordinate> {
        let neighbours: Array2<i32> = arr2(
            &[[-1, -1], [0, -1], [1, -1],
                [-1, 0], [0, 0], [1, 0],
                [-1, 1], [0, 1], [1, 1]]);

        let ones: Array2<i32> = Array2::<i32>::ones((9, 2));

        let coord_vector = arr2(
            &[[coord.x, 0],
                [0, coord.y]]);

        let cellmates: Array2<i32> = ones.dot(&coord_vector) + neighbours;

        cellmates.outer_iter().map(|row| Coordinate { x: row[0], y: row[1] }).collect()
    }

    pub fn get_interesting_neighbours(game: &HashMap<Coordinate, Cell>, interesting: &mut HashSet<Coordinate>) {
        let neighbour_sets = game.iter().map(|(coord, _)| get_neighbours(coord));

        neighbour_sets.for_each(
            |interesting_neighbour_set| {
                interesting.extend(interesting_neighbour_set.iter().filter(|c| c.x >= 0 && c.y >= 0));
            });
    }

    pub fn next(current: &HashMap<Coordinate, Cell>, future: &mut HashMap<Coordinate, Cell>) {
        let mut interesting = HashSet::new();
        get_interesting_neighbours(current, &mut interesting);

        interesting.iter().for_each(|coord|
            match count_living_neighbours(current, coord) {
                2 => match current.get(coord) {
                    Some(v) => {
                        future.insert(*coord, *v);
                        ()
                    }
                    None => ()
                },
                3 => bring_to_life(future, coord),
                _ => ()
            })
    }

    pub fn count_living_neighbours(game: &HashMap<Coordinate, Cell>, coord: &Coordinate) -> usize {
        get_neighbours(coord).iter().filter(|c| is_alive(game, c)).count()
    }

    pub fn bring_to_life(game: &mut HashMap<Coordinate, Cell>, coordinate: &Coordinate) {
        game.insert(*coordinate, Cell { alive: true });
    }

    pub fn kill(game: &mut HashMap<Coordinate, Cell>, coordinate: &Coordinate) {
        game.remove(coordinate);
    }
}