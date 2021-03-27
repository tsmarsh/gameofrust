#[cfg(test)]
mod tests {
    use crate::parse::*;
    use crate::gol::{Game, World};
    use std::collections::LinkedList;

    #[test]
    fn follows_a_plan() {
        let seq = ["....\n\
                           ..0.\n\
                           .0..\n\
                           .0..\n\
                           ....",

                           "....\n\
                            ....\n\
                           .00.",
                           ""];

        let mut worlds: LinkedList<Game> = seq.iter().map(|s| parse_world(& s)).collect();

        let start = worlds.pop_front().unwrap();

        let results: Vec<World> = start.take(worlds.len()).collect();

        results.iter().for_each(|r|
            assert_eq!(r, &worlds.pop_front().unwrap().world));
    }

    #[test]
    fn can_serialize_a_world() {
        let world = ".....\n\
                       ...0.\n\
                       .0.0.\n\
                       ..00.\n";

        let game = parse_world(world);

        assert_eq!(world, format!("{:?}", game));
    }
}