pub fn encontrar_kesimo_maior(nums: &mut Vec<i32>, k: usize) -> i32 {
    nums.sort_by(|a, b| b.cmp(a));
    return nums[k-1]
}