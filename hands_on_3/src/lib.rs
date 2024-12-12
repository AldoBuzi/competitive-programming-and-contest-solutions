pub fn solution(m: usize, matrix: Vec<Vec<i32>>) -> i32 {
    //squeeze matrix to two arrays
    let mut temporay = vec![0; m];
    let mut result = vec![0; m];
    for (index_row, row) in matrix.iter().enumerate() {
        let mut row_sum = 0;
        for (index_col, col) in row.iter().enumerate() {
            row_sum += *col;
            if index_row == 0 {
                result[index_col] = row_sum;
            } else {
                let mut sum = 0;
                let mut max = 0;
                for (index_key, key) in row.iter().enumerate() {
                    sum += *key;
                    if index_key >= index_col {
                        break;
                    }
                    let index: usize = index_col - index_key - 1;
                    max = max.max(temporay[index] + sum)
                }
                // max (cell above, row sum, max (all combinations of prefix and solution on the cell above) )
                result[index_col] = temporay[index_col].max(max).max(sum);
            }
        }
        temporay = result.clone();
    }
    result[m - 1]
}

pub fn solution2(pairs: &mut [(usize, usize)]) -> usize {
    if pairs.is_empty() {
        return 0;
    }
    //solve first requirement about increasing difficulties
    //After this, the problem is reduced to solve a "longest increasing subsequence(LIS)" problem
    pairs.sort_unstable_by_key(|&x| x.1);
    let mut res = Vec::<(usize, usize)>::new();
    res.push(pairs[0]);
    // for in range 1..pairs.len()
    for &pair in pairs.iter().skip(1) {
        // pair.1's difficulty is alwasy greater or equal than the second one
        // I added this to avoid cases like (50,10) compared against (40,10)
        // We don't want to add 50 at all.
        let last = res.last().unwrap();
        if pair.0 > last.0 && pair.1 != last.1 {
            res.push(pair);
        }
        // this if avoids adding (50,10), but allows adding an item such as (30,10)
        else if pair.0 <= last.0 {
            let mut low = 0;
            let mut high = res.len() - 1;
            while low < high {
                let mid = low + (high - low) / 2;
                if res[mid].0 < pair.0 {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }
            res[low] = pair;
        }
    }
    res.len()
}
