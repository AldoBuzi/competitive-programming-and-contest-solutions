use std::collections::VecDeque;


pub fn solution(nums: &Vec<i32>) -> Vec<i32> {
    let n = nums.len();

    let mut q: VecDeque<usize> = VecDeque::new();
    let mut maxs: Vec<i32> = Vec::with_capacity(n);

    for i in 0..n {
        while !q.is_empty() && nums[i] > nums[*q.front().unwrap()] {
            maxs.push(nums[i]);
            q.pop_front();
        }
        q.push_back(i);

    }
    maxs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vec = vec![3,1,4,5,2,1,8];
        let result = solution(&vec);
        assert_eq!(result, vec![4,4,5,8,8,8]);
    }
    #[test]
    fn test_2() {
        let vec = vec![3,1,3,1,3,4,5,2,1,8];
        let result = solution(&vec);
        assert_eq!(result, vec![4,4,4,4,4,5,8,8,8]);
    }
}
