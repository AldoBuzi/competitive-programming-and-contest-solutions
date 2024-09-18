use std::cmp;
fn main() {
    let input = vec!(3,0,0,2,0,4);

    let expected_output = 10;

    if trapping_rain_water(input) == expected_output {
        println!("The program works properly. The value is {}", expected_output)
    };
    
}

fn trapping_rain_water(values: Vec<i32> /* values are not negative */)  -> i32 {
    let area = match (values.first(), values.last()) {
        (Some(first),Some(last)) => Some( cmp::min(first, last) * values.len() as i32 + last - first),
        _ => None
    };
    area.unwrap_or(0) - values.iter().map(|&i| i as i32).sum::<i32>()
}