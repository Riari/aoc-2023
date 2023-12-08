pub mod template;

use num::integer::lcm;

/// Least Common Multiple of a vector of numbers
pub fn lcm_of_vec(numbers: Vec<u64>) -> u64 {
    if numbers.is_empty() {
        return 0;
    }

    let mut result = numbers[0];
    for &num in &numbers[1..] {
        result = lcm(result, num);
    }
    result
}
