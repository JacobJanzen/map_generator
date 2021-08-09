use crate::Map;
use rand::prelude::*;
use rand_pcg::Pcg64;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const MIN_KEEP_WALL: u8 = 4;
const MIN_NEW_WALL: u8 = 5;

impl Map {
    fn next_cellular_automata(&self) -> Map {
        let mut new_map = Map::new(self.height, self.width);

        for i in 0..self.height {
            for j in 0..self.width {
                new_map.set(i, j, self.calculate_new_cell(i, j));
            }
        }

        new_map
    }

    fn cleanup_cellular_automata(&self) -> Map {
        let mut new_map = Map::new(self.height, self.width);

        for i in 0..self.height {
            for j in 0..self.width {
                new_map.set(i, j, self.cleanup_cell(i, j));
            }
        }

        new_map
    }

    fn cleanup_cell(&self, y: usize, x: usize) -> bool {
        let num_neighbours = self.count_neighbours(y, x);

        if num_neighbours >= MIN_NEW_WALL {
            return true;
        }
        if num_neighbours == MIN_KEEP_WALL && self.get(y, x) {
            return true;
        }
        false
    }

    fn calculate_new_cell(&self, y: usize, x: usize) -> bool {
        let num_neighbours = self.count_neighbours(y, x);

        if num_neighbours >= MIN_NEW_WALL || self.empty_space(y, x) {
            return true;
        }
        if num_neighbours == MIN_KEEP_WALL && self.get(y, x) {
            return true;
        }

        false
    }

    fn empty_space(&self, y: usize, x: usize) -> bool {
        if self.count_far_neighbours(y, x) == 0 {
            return true;
        }
        false
    }

    fn count_far_neighbours(&self, y: usize, x: usize) -> u8 {
        let mut total = self.count_neighbours(y, x);

        if x <= 1 || y <= 1 || self.get(y - 2, x - 2) {
            total += 1;
        }
        if x == 0 || y <= 1 || self.get(y - 2, x - 1) {
            total += 1;
        }
        if y <= 1 || self.get(y - 2, x) {
            total += 1;
        }
        if y <= 1 || self.get(y - 2, x + 1) {
            total += 1;
        }
        if y <= 1 || self.get(y - 2, x + 2) {
            total += 1;
        }
        if y == 0 || self.get(y - 1, x + 2) {
            total += 1;
        }
        if self.get(y, x + 2) {
            total += 1;
        }
        if self.get(y + 1, x + 2) {
            total += 1;
        }
        if self.get(y + 2, x + 2) {
            total += 1;
        }
        if self.get(y + 2, x + 1) {
            total += 1;
        }
        if self.get(y + 2, x) {
            total += 1;
        }
        if x == 0 || self.get(y + 2, x - 1) {
            total += 1;
        }
        if x <= 1 || self.get(y + 2, x - 2) {
            total += 1;
        }
        if x <= 1 || self.get(y + 1, x - 2) {
            total += 1;
        }
        if x <= 1 || self.get(y, x - 2) {
            total += 1;
        }
        if x <= 1 || y == 0 || self.get(y - 1, x - 2) {
            total += 1;
        }

        total
    }

    fn count_neighbours(&self, y: usize, x: usize) -> u8 {
        let mut neighbours = 0;

        if x == 0 || y == 0 || self.get(y - 1, x - 1) {
            neighbours += 1;
        }
        if y == 0 || self.get(y - 1, x) {
            neighbours += 1;
        }
        if y == 0 || self.get(y - 1, x + 1) {
            neighbours += 1;
        }
        if self.get(y, x + 1) {
            neighbours += 1;
        }
        if self.get(y + 1, x + 1) {
            neighbours += 1;
        }
        if self.get(y + 1, x) {
            neighbours += 1;
        }
        if x == 0 || self.get(y + 1, x - 1) {
            neighbours += 1;
        }
        if x == 0 || self.get(y, x - 1) {
            neighbours += 1;
        }

        neighbours
    }

    /// Creates a cave based on a given seed
    ///
    /// # Examples
    ///
    /// ```
    /// use map_generator::Map;
    ///
    /// let map = Map::gen_cave_seed(10,10,String::from("0"));
    /// assert!(map.get(0,0));
    /// assert!(!map.get(3,2));
    /// ```
    pub fn gen_cave_seed(y: usize, x: usize, seed: String) -> Map {
        let mut s = DefaultHasher::new();

        let seed: u64 = match seed.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                seed.hash(&mut s);
                s.finish()
            }
        };

        let mut rng = Pcg64::seed_from_u64(seed);
        Map::gen_cave(y, x, &mut rng)
    }

    /// Creates a cave without providing a seed
    ///
    /// # Examples
    ///
    /// ```
    /// use map_generator::Map;
    ///
    /// let map = Map::gen_cave_no_seed(10,10);
    /// ```
    pub fn gen_cave_no_seed(y: usize, x: usize) -> Map {
        let mut rng = rand::thread_rng();
        Map::gen_cave(y, x, &mut rng)
    }

    fn gen_cave<T: Rng>(y: usize, x: usize, rng: &mut T) -> Map {
        let mut map = Map::new(y, x);
        map.fill_random(rng);
        for _ in 0..5 {
            map = map.next_cellular_automata();
        }

        map = map.cleanup_cellular_automata();

        map
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn correct_neighbour_count() {
        let map = Map::new(1, 1);
        let num_neighbours = map.count_neighbours(0, 0);
        assert_eq!(8, num_neighbours);

        let mut map = Map::new(3, 3);
        let num_neighbours = map.count_neighbours(1, 1);
        assert_eq!(0, num_neighbours);

        map.set(0, 0, true);
        let num_neighbours = map.count_neighbours(1, 1);
        assert_eq!(1, num_neighbours);

        map.set(0, 1, true);
        let num_neighbours = map.count_neighbours(1, 1);
        assert_eq!(2, num_neighbours);

        map.set(0, 2, true);
        let num_neighbours = map.count_neighbours(1, 1);
        assert_eq!(3, num_neighbours);

        map.set(1, 2, true);
        let num_neighbours = map.count_neighbours(1, 1);
        assert_eq!(4, num_neighbours);

        map.set(2, 2, true);
        let num_neighbours = map.count_neighbours(1, 1);
        assert_eq!(5, num_neighbours);

        map.set(2, 1, true);
        let num_neighbours = map.count_neighbours(1, 1);
        assert_eq!(6, num_neighbours);

        map.set(2, 0, true);
        let num_neighbours = map.count_neighbours(1, 1);
        assert_eq!(7, num_neighbours);

        map.set(1, 0, true);
        let num_neighbours = map.count_neighbours(1, 1);
        assert_eq!(8, num_neighbours);
    }

    #[test]
    fn correct_new_char() {
        let map = Map::new(1, 1);
        assert!(map.calculate_new_cell(0, 0));

        let mut map = Map::new(3, 3);
        assert!(!map.calculate_new_cell(1, 1));

        map.set(0, 0, true);
        assert!(!map.calculate_new_cell(1, 1));

        map.set(0, 1, true);
        map.set(0, 2, true);
        map.set(1, 2, true);
        assert!(!map.calculate_new_cell(1, 1));

        map.set(1, 1, true);
        assert!(map.calculate_new_cell(1, 1));

        map.set(1, 1, false);
        map.set(2, 2, true);
        assert!(map.calculate_new_cell(1, 1));
    }

    #[test]
    fn generate_map() {
        let map = Map::gen_cave_seed(10, 10, String::from("0"));

        let map_string = format!("{}", map);
        println!("{}", map);

        let expected_map_string = String::from(
            "\
############
############
############
###...######
##......####
##......####
##......####
##.....#####
##....######
###..#######
############
############",
        );

        assert_eq!(expected_map_string, map_string);
    }
}
