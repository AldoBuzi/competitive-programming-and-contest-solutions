use std::collections::BTreeSet;

fn main() {
    let values = vec![1,3,-1,-3,5,3,6,7];
    let k = 3;
    trivial_solution(&values, k);
    bst_solution(&values, k);
}

fn trivial_solution(values: &Vec<i32>, k: usize) {
    let mut max : i32 = -1;
    for key in 0..values.len() - k{
        let value = values[key .. key + k].iter().max().unwrap();
        if *value > max {
            max = *value;
        }
    }
    println!("Max value is {}",max)
}

fn bst_solution(values: &Vec<i32>, k: usize){
    if  k > values.len(){
        return; 
    }
    let mut bst = BTreeSet::new();
    let mut set = Vec::with_capacity(values.len() - k + 1);
    
    for (index, &value) in values.iter().enumerate(){
        if index > k  {
            set.push(*bst.last().unwrap()); 
            bst.remove(&values[index-k]);
        }
        bst.insert(value);

    }
    set.push(*bst.last().unwrap()); 

    println!("Resulting array {}",join_nums(&set,","));

    // Solution O(n log k) --> worst case if k = n, then O(n logn)

}

fn join_nums(nums: &[i32], sep: &str) -> String {
    // 1. Convert numbers to strings
    let str_nums: Vec<String> = nums.iter() 
        .map(|n| n.to_string())  // map every integer to a string
        .collect();  // collect the strings into the vector
    
    // 2. Join the strings. There's already a function for this.
    str_nums.join(sep)
}