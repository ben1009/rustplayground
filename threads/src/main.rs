use std::collections::HashMap;

struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            map.insert(*n, i);
        }

        for (i, n) in nums.iter().enumerate() {
            if let Some(v) = map.get_key_value(&(target - n)) {
                if *v.1 == i {
                    continue;
                }

                return vec![*v.1 as i32, i as i32];
            }
        }

        vec![]
    }
}

fn main() {}
