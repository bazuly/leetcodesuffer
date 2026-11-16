impl Solution {
    pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
        let mut n = nums.len();
        let mut even = 0;
        let mut odd = 1;

        while even < n && odd < n {
            while even < n && nums[even] % 2 == 0 {
                even += 2;
            }
            while odd < n && nums[odd] % 2 != 0 {
                odd += 2;
            }

            if even < n && odd < n {
                nums.swap(even, odd);
                even += 2;
                odd += 2;
            }
        }
        nums
    }
}
