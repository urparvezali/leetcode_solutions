use std::collections::BTreeSet;
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut set = BTreeSet::new();
        for &num in &nums {
            if let Some(&x) = set.range(num..).next() {
                set.remove(&x);
            }
            set.insert(num);
        }
        set.len() as i32
    }
}
