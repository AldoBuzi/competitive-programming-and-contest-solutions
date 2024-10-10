use std::cmp::Ordering;

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let i = nums.binary_search_by(|n| if n < &target { Ordering::Less } else { Ordering::Greater }).unwrap_err();
        
    if i == nums.len() || nums[i] != target {
        return vec![-1, -1];   
    }
        
    let j = nums.binary_search_by(|n| if n <= &target { Ordering::Less } else { Ordering::Greater }).unwrap_err();
        
    vec![i as i32, (j - 1) as i32]      
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vec0  = vec![5,7,7,8,8,10];
        let result = search_range(vec0, 8);
        assert_eq!(result, [3,4]);
    }

    #[test]
    fn test1() {
        let vec0  = vec![5,7,7,8,8,10];
        let result = search_range(vec0, 6);
        assert_eq!(result, [-1,-1]);
    }

    #[test]
    fn test2() {
        let vec0  = vec![];
        let result = search_range(vec0, 8);
        assert_eq!(result, [-1,-1]);
    }

    #[test]
    fn test3() {
        let vec0  = vec![1,2,3,4,5,6,7,8,8,8,8,8,8,8,10];
        let result = search_range(vec0, 8);
        assert_eq!(result, [7,13]);
    }

    #[test]
    fn test4() {
        let vec0  = vec![1,1,1,1,1,1,2,3,4,5,6];
        let result = search_range(vec0, 1);
        assert_eq!(result, [0,5]);
    }
}
