use std::collections::HashSet;

pub fn is_palyndrome(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false
    }

    let mut original = x;
    let mut invertido = 0;

    while original != 0 {
        let digito = original % 10;
        invertido = invertido * 10 + digito;
        original /= 10;

    }
    return x == invertido
}

pub fn move_zeros(nums: &mut Vec<i32>) {
    let mut indice_nao_zero = 0;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums.swap(i, indice_nao_zero);
            indice_nao_zero += 1;
        }
    }
}

pub fn contains_duplicate(nums: &[i32]) -> bool {
    let mut seen = HashSet::new();
    for &num in nums{
        if !seen.insert(num) {
            return true;
        }
    }
    return false;
}
