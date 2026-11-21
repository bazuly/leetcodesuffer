impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let n = arr.len();
        let mut output = Vec::new();

        for num in arr.iter() {
            output.push(num);
            if num == 0 {
                output.push(0);
            }
        }
        for i in 0..n {
            arr[i] = output[i]
        }
    }
}
