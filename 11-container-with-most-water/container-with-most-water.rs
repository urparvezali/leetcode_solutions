impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut area = 0;
        let mut a = 0;
        let mut ptr = (0, 0);
        let (mut l, mut r) = (0, height.len()-1);

        while(l<r){
            a = height[l].min(height[r])*(r-l) as i32;
            area = area.max(a);
            if height[l]<height[r] {
                l+=1;
            } else {
                r-=1;
            }
        }
        area
    }
}