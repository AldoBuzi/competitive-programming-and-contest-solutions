use std::cmp::Ordering::{Greater, Less, Equal};


pub fn find_min(nums: Vec<i32>) -> i32 {
    let n = nums.len();
        let k = find_peak(&nums);
        nums[k%n]
}
fn find_peak(nums: &Vec<i32>) -> usize {
    let (mut lbound, mut ubound) = (0, nums.len());
    while lbound < ubound {
        let mid = (lbound+ubound)/2;
        match nums[mid].cmp(&nums[0]) {
            Greater|Equal => {lbound = mid+1;}
            Less => {ubound = mid;}
        }
    }
    ubound
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn it_works() {
        let vec = vec![10,3,4,5,6,7,8];
        let result = find_min(vec);
        assert_eq!(result, 3);
    }

    #[test]
    fn test1() {
        env::set_var("RUST_BACKTRACE", "1");
        let vec = vec![4,5,6,7,0,1,2];
        let result = find_min(vec);
        assert_eq!(result, 0);
    }

    #[test]
    fn test2() {
        let vec = vec![2,3,4,5,6,7,1];
        let result = find_min(vec);
        assert_eq!(result, 1);
    }
    #[test]
    fn test3() {
        let vec = vec![11,12,6,7,8,9,10];
        let result = find_min(vec);
        assert_eq!(result, 6);
    }
}
