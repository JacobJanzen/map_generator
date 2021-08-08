use rand::Rng;
use std::fmt;

pub struct Map {
    map: Vec<bool>,
    pub height: usize,
    pub width: usize,
}

impl Map {
    pub fn new(height: usize, width: usize) -> Map {
        Map {
            map: vec![false; width * height],
            width,
            height,
        }
    }

    pub fn get(&self, y: usize, x: usize) -> bool {
        if y >= self.height || x >= self.width {
            // anything outside of the map is a wall
            return true;
        }

        // otherwise return the value at the given position
        self.map[y * self.width + x]
    }

    pub fn set(&mut self, y: usize, x: usize, new_val: bool) {
        if y < self.height && x < self.width {
            self.map[y * self.width + x] = new_val;
        }
    }

    pub fn next(&self) -> Map {
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

        if num_neighbours >= 5 {
            return true;
        }
        if num_neighbours == 4 && self.get(y, x) {
            return true;
        }

        false
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

    pub fn fill_random(&mut self) {
        let mut rng = rand::thread_rng();

        for i in 0..self.height {
            for j in 0..self.width {
                self.set(i, j, rng.gen_bool(0.5));
            }
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.get(i, j) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
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
    fn display() {
        let map = Map::new(5, 5);

        let map_string = format!("{}", map);

        let expected_map_string = String::from(
            "\
.....
.....
.....
.....
.....
",
        );

        assert_eq!(expected_map_string, map_string);
    }
}
