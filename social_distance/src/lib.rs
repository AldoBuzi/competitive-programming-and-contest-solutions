use num::FromPrimitive;
use num::Num;
use std::cmp::PartialOrd;

pub fn binary_search_range<T, F>(low: T, high: T, pred: F) -> Option<T>
where
    T: Num + PartialOrd + FromPrimitive + Copy,
    F: Fn(T) -> bool,
{
    let mut low = low;
    let mut high = high;

    let mut ans = None;

    while low < high {
        
        let middle = low + (high - low) / FromPrimitive::from_u64(2).unwrap();
        match pred(middle) {
            true => {
                low = middle + T::one();
                ans = Some(middle)
            }
            false => high = middle,
        }
    }
    ans
}
//Expected complexity: theta(nlogn)
pub fn select_intervals(intervals: &mut Vec<(usize, usize)>, c: usize) -> Option<usize> {
    let length = intervals.iter().fold(0, |acc, interval| acc + interval.1 - interval.0 + 1 );
    print!("My value {}\n", length);
    if length < c {
        return None;
    }
    print!("My value {}\n", length);
    let pred = |d:usize| -> bool {
        let mut last_selected = intervals[0].0;
        let mut cnt = 1;
        for &interval in intervals.iter() {
            while interval.0.max(last_selected + d) <= interval.1 {
                last_selected = interval.0.max(last_selected + d);
                cnt += 1;
            }
        }
        cnt >= c
    };

    binary_search_range(1, length + 1, pred )
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    #[test]
    fn it_works() {
        env::set_var("RUST_BACKTRACE", "1");
        let mut my_vec : Vec<(usize,usize)> = vec![(0,2),(4,7),(9,9)];
        let result = select_intervals(&mut my_vec, 5);
        match result {
          Some(value) => assert_eq!(value, 2),
          None => assert_eq!(true, false)
        }
    }
    #[test]
    fn it_works2() {
        env::set_var("RUST_BACKTRACE", "1");
        let mut my_vec : Vec<(usize,usize)> = vec![(0,2),(4,7),(9,9), (10,13)];
        let result = select_intervals(&mut my_vec, 8);
        match result {
          Some(value) => assert_eq!(value, 1),
          None => assert_eq!(true, false)
        }
    }
}
