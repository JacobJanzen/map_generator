use rand::prelude::*;
use rand_pcg::Pcg64;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

const WALL: char = '#';
const FLOOR: char = '.';
const INIT_PROBABILITY: f64 = 0.45;
const MIN_KEEP_WALL: u8 = 4;
const MIN_NEW_WALL: u8 = 5;

pub struct Map {
    map: Vec<bool>,
    pub height: usize,
    pub width: usize,
}

impl Map {
    /// Create empty map
    ///
    /// # Examples
    ///
    /// ```
    /// use map_generator::Map;
    ///
    /// let map = Map::new(100,50);
    ///
    /// assert_eq!(100, map.height);
    /// assert_eq!(50, map.width);
    /// ```
    pub fn new(height: usize, width: usize) -> Map {
        Map {
            map: vec![false; width * height],
            width,
            height,
        }
    }

    /// Check if a wall is present at a given position.
    ///
    /// # Examples
    ///
    /// ```
    /// use map_generator::Map;
    ///
    /// let map = Map::gen_cave_seed(10,10,String::from("0"));
    /// assert!(map.get(0,0));
    /// assert!(!map.get(3,2));
    ///
    /// // anything outside of the map is a wall
    /// assert!(map.get(100,100));
    /// ```
    pub fn get(&self, y: usize, x: usize) -> bool {
        if y >= self.height || x >= self.width {
            // anything outside of the map is a wall
            return true;
        }

        // otherwise return the value at the given position
        self.map[y * self.width + x]
    }

    /// Set the value at a given position in the map
    ///
    /// # Examples
    ///
    /// ```
    /// use map_generator::Map;
    ///
    /// let mut map = Map::new(10,10);
    /// assert!(!map.get(0,0));
    ///
    /// map.set(0,0,true);
    /// assert!(map.get(0,0));
    /// ```
    pub fn set(&mut self, y: usize, x: usize, new_val: bool) {
        if y < self.height && x < self.width {
            self.map[y * self.width + x] = new_val;
        }
    }

    fn next_cellular_automata(&self) -> Map {
        let mut new_map = Map::new(self.height, self.width);

        for i in 0..self.height {
            for j in 0..self.width {
                new_map.set(i, j, self.calculate_new_cell(i, j));
            }
        }

        new_map
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

    fn fill_random<T: Rng>(&mut self, rng: &mut T) {
        for i in 0..self.height {
            for j in 0..self.width {
                self.set(i, j, rng.gen_bool(INIT_PROBABILITY));
            }
        }
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

        map
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for _ in 0..self.width + 2 {
            write!(f, "{}", WALL)?;
        }
        writeln!(f)?;
        for i in 0..self.height {
            write!(f, "{}", WALL)?;
            for j in 0..self.width {
                if self.get(i, j) {
                    write!(f, "{}", WALL)?;
                } else {
                    write!(f, "{}", FLOOR)?;
                }
            }
            writeln!(f, "{}", WALL)?;
        }
        for _ in 0..self.width + 2 {
            write!(f, "{}", WALL)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_map() {
        let map = Map::new(100, 50);

        assert_eq!(100, map.height);
        assert_eq!(50, map.width);
    }

    #[test]
    fn create_empty_map() {
        let map = Map::new(0, 0);

        assert_eq!(0, map.height);
        assert_eq!(0, map.width);
    }

    #[test]
    fn get_value_in_bounds() {
        let map = Map::new(10, 10);

        assert!(!map.get(0, 0));
    }

    #[test]
    fn get_value_out_of_bounds() {
        let map = Map::new(10, 10);

        assert!(map.get(100, 100));
    }

    #[test]
    fn set_value() {
        let mut map = Map::new(10, 10);

        map.set(0, 0, true);

        assert!(map.get(0, 0));

        map.set(0, 0, false);

        assert!(!map.get(0, 0));
    }

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
##.......###
##......####
##.....#####
##....######
###..#######
############
############",
        );

        assert_eq!(expected_map_string, map_string);
    }

    #[test]
    fn display() {
        let map = Map::new(5, 5);

        let map_string = format!("{}", map);

        let expected_map_string = String::from(
            "\
#######
#.....#
#.....#
#.....#
#.....#
#.....#
#######",
        );

        assert_eq!(expected_map_string, map_string);
    }
}
