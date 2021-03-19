#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use crate::gol::*;

    #[test]
    fn a_cell_comes_to_life() {
        let mut game: HashMap<Coordinate, Cell> = HashMap::new();
        let home = Coordinate { x: 0, y: 0 };

        assert_eq!(false, is_alive(&game, &home));
        bring_to_life(&mut game, &home);

        assert_eq!(true, is_alive(&game, &home));
    }

    #[test]
    fn a_cell_can_die() {
        let mut game: HashMap<Coordinate, Cell> = HashMap::new();
        let home = Coordinate { x: 0, y: 0 };

        bring_to_life(&mut game, &home);
        kill(&mut game, &home);
        assert_eq!(false, is_alive(&game, &home));
    }

    #[test]
    fn a_dead_cell_with_three_neighbours_comes_to_life() {
        let mut game: HashMap<Coordinate, Cell> = HashMap::new();
        let mut game_prime: HashMap<Coordinate, Cell> = HashMap::new();
        let buddy_1 = Coordinate { x: 0, y: 0 };
        let buddy_2 = Coordinate { x: 2, y: 0 };
        let buddy_3 = Coordinate { x: 1, y: 1 };
        let subject = Coordinate { x: 1, y: 0 };

        bring_to_life(&mut game, &buddy_1);
        bring_to_life(&mut game, &buddy_2);
        bring_to_life(&mut game, &buddy_3);

        assert_eq!(false, is_alive(&game, &subject));

        next(&game, &mut game_prime);

        assert_eq!(true, is_alive(&game_prime, &subject));
    }

    #[test]
    fn should_list_neighbours_of_a_coordinate() {
        let coord = Coordinate {
            x: 2,
            y: 3,
        };

        let neighbours = get_neighbours(&coord);
        assert_eq!(true, neighbours.contains(&Coordinate { x: 2, y: 3 }));
        assert_eq!(true, neighbours.contains(&Coordinate { x: 3, y: 4 }));
        assert_eq!(true, neighbours.contains(&Coordinate { x: 1, y: 2 }));
    }

    #[test]
    fn should_compile_all_interesting_coordinates() {
        let mut game: HashMap<Coordinate, Cell> = HashMap::new();
        bring_to_life(&mut game, &Coordinate { x: 0, y: 0 });
        bring_to_life(&mut game, &Coordinate { x: 2, y: 0 });

        let mut interesting = HashSet::new();
        let mut expected: HashSet<Coordinate> = HashSet::new();

        expected.insert(Coordinate { x: 1, y: 0 });
        expected.insert(Coordinate { x: 3, y: 0 });
        expected.insert(Coordinate { x: 3, y: 1 });
        expected.insert(Coordinate { x: 0, y: 0 });
        expected.insert(Coordinate { x: 0, y: 1 });
        expected.insert(Coordinate { x: 2, y: 1 });
        expected.insert(Coordinate { x: 1, y: 1 });
        expected.insert(Coordinate { x: 2, y: 0 });


        get_interesting_neighbours(&game, &mut interesting);

        assert_eq!(expected, interesting);
    }

    #[test]
    fn should_count_living_neighbours() {
        let mut game: HashMap<Coordinate, Cell> = HashMap::new();
        let neighbours =
            vec![Coordinate { x: 0, y: 0 },
                 Coordinate { x: 2, y: 0 }];

        neighbours.iter().for_each(|n| bring_to_life(&mut game, n));
        assert_eq!(2, count_living_neighbours(&game, &Coordinate { x: 1, y: 1 }));
    }
}