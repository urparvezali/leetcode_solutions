impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![1; n];
        let mut mx = 0;
        for i in (0..n).rev() {
            mx = 0;
            for j in i + 1..n {
                if nums[i] < nums[j] {
                    mx = mx.max(dp[j]);
                }
            }
            dp[i] += mx;
        }
        *dp.iter().max().unwrap_or(&1)
    }
}