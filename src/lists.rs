use std::collections::HashSet;

pub fn rotacionar_array(nums: &mut [i32; 7], k: usize) {
    let n = nums.len();
    if n == 0 {
        return;
    }

    let k = k % n;
    nums.reverse();
    nums[0..k].reverse();
    nums[k..].reverse();

}

pub fn interseccao(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let conjunto1: HashSet<_> = nums1.into_iter().collect();
    let conjunto2: HashSet<_> = nums2.into_iter().collect();

    let resultado: Vec<_> = conjunto1.intersection(&conjunto2).cloned().collect();
    resultado
}

pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort();

    let mut closest_sum = i32::MAX;
    let mut min_diff = i32::MAX;

    for i in 0..(nums.len()-2) {
        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let current_sum = nums[i] + nums[left] + nums[right];
            let current_diff = (target - current_sum).abs();

            if current_diff < min_diff {
                closest_sum = current_sum;
                min_diff = current_diff;
            }

            if current_sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }
    closest_sum    
}

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut resultado = Vec::new();
    let mut subset_atual = Vec::new();

    fn backtrack(nums: &Vec<i32>, inicio: usize, subset_atual: &mut Vec<i32>, resultado: &mut Vec<Vec<i32>>) {
        resultado.push(subset_atual.clone());
        for i in inicio..nums.len() {
            subset_atual.push(nums[i]);
            backtrack(nums, i + 1, subset_atual, resultado);
            subset_atual.pop();
        }
    }

    backtrack(&nums, 0, &mut subset_atual, &mut resultado);
    resultado
}