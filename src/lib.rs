mod cave;
use rand::prelude::*;
use std::fmt;

const WALL: char = '#';
const FLOOR: char = '.';
const INIT_PROBABILITY: f64 = 0.45;

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

    fn fill_random<T: Rng>(&mut self, rng: &mut T) {
        for i in 0..self.height {
            for j in 0..self.width {
                self.set(i, j, rng.gen_bool(INIT_PROBABILITY));
            }
        }
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
