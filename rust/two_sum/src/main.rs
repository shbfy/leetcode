fn main() {
    println!("{:?}", Solution::two_sum_brute(vec![2, 7, 11, 15], 9))
}

struct Solution;

impl Solution {
    // Brute force approach
    pub fn two_sum_brute(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (idx1, &num1) in nums.iter().enumerate() {
            for (idx2, &num2) in nums.iter().enumerate() {
                if num1 + num2 == target && idx1 != idx2 {
                    return vec![idx1 as i32, idx2 as i32];
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum_brute(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum_brute(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum_brute(vec![3, 3], 6), vec![0, 1]);
        assert_eq!(Solution::two_sum_brute(vec![3, 2, 4], 10), vec![]);
    }
}
