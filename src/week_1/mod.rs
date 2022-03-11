use std::collections::HashMap;

//给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target 的那 两个 整数，并返回它们的数组下标。
//
// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
//
// 你可以按任意顺序返回答案。
//
//
//
// 示例 1：
//
//
//输入：nums = [2,7,11,15], target = 9
//输出：[0,1]
//解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。
//
//
// 示例 2：
//
//
//输入：nums = [3,2,4], target = 6
//输出：[1,2]
//
//
// 示例 3：
//
//
//输入：nums = [3,3], target = 6
//输出：[0,1]
//
//
//
//
// 提示：
//
//
// 2 <= nums.length <= 10⁴
// -10⁹ <= nums[i] <= 10⁹
// -10⁹ <= target <= 10⁹
// 只会存在一个有效答案
//
//
// 进阶：你可以想出一个时间复杂度小于 O(n²) 的算法吗？
// Related Topics 数组 哈希表 👍 13688 👎 0

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

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        let v = vec![1,2,3,4,5];
        let d = 6;
        let vec1 = super::test(v, d);
        println!("{:?}", vec1);
    }
}