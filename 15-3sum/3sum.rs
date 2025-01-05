use std::collections::HashSet;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut ans = Vec::new();
        let mut set = HashSet::new();

        nums.sort();

        for i in 0..nums.len() - 2 {
            let mut lo = i + 1;
            let mut hi = nums.len() - 1;

            while lo < hi {
                let sum = nums[i] + nums[lo] + nums[hi];

                if sum == 0 {
                    let triplet = vec![nums[i], nums[lo], nums[hi]];
                    if set.insert(triplet.clone()) {
                        ans.push(triplet);
                    }
                    lo += 1;
                    hi -= 1;
                } else if sum < 0 {
                    lo += 1;
                } else {
                    hi -= 1;
                }
            }
        }

        ans
    }
}
