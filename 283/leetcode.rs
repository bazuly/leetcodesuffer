impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut k = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(k, i);
                k += 1;
            }
        }
    }
}

// без свап через временную переменную
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut k = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                let tmp = nums[k];
                nums[k] = nums[i];
                nums[i] = tmp;
                k += 1;
            }
        }
    }
}
