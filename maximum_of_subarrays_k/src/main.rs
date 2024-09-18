fn main() {
    let values = vec![3,5,6,7,2,1,6,23,2,6,2];
    let k = 3;
    solution(&values, &k);
}

fn solution(values: &Vec<i32>, k: &i32) {
    let mut max: i32 = -1;
    let mut actual_size = 0;
    values.iter().for_each(|&value| {
        if actual_size == *k {
            actual_size = 0;
            println!("The maximum value is {}",max);
            max = -1
        }
        if max <= value {
            max = value
        }
        actual_size += 1;
    });
    if actual_size != 0 {
        println!("The maximum value is {}",max);
    }
}