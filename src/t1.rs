use std::collections::HashMap;

pub fn test(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_map = HashMap::<i32, usize>::new();

    for (index, &value) in nums.iter().enumerate() {
        if let Some(&t) = hash_map.get(&(target - value)) {
            return  vec![t as i32, index as i32];
        }
        hash_map.insert(value, index);
    }


    // for i in 0..len {
    //     if hash_map.contains_key(&(target - nums[i])) {
    //         let d = hash_map.get(&(target - nums[i])).unwrap();
    //         return vec![*d, i as i32];
    //     }
    //     hash_map.insert(nums[i], i as i32);
    // }

    vec![]
}
