use polars::prelude::*;
use rand::prelude::*;


pub fn create_shuffled_series(students: &DataFrame) -> Series {
// make a vector 1 to n and shuffle the numbers
    let n_students: u32 = students.shape().0 as u32;
    let nums = create_shuffled_vector(n_students);
    let nums = Series::new("x", nums);
    nums
}

fn create_shuffled_vector(n_students: u32) -> Vec<u32> {
    let mut rng = thread_rng();
    let mut nums: Vec<u32> = Vec::from_iter(std::ops::Range {
        start: 0,
        end: n_students,
    });
    nums.shuffle(&mut rng);
    nums
}


#[cfg(test)]
mod tests {
    use crate::{create_shuffled_vector};

    #[test]
    fn test_create_shuffled_vector() {
        let x = create_shuffled_vector(10);
        let y = create_shuffled_vector(10);
        assert_ne!(x, y)
    }
}