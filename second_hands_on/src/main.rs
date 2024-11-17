use std::collections::{HashSet};

#[cfg(test)]
mod test_cases_reader;

pub struct SegmentTree {
    array: Vec<HashSet<usize>>,
    end: usize
}

impl SegmentTree {
    pub fn build(values: Vec<(usize,usize)>, n: usize) -> Self {
        let input_size = 2usize.pow((n as f32).log2().ceil() as u32);
        let tree_size = 2 * input_size - 1;
        let mut pairs: Vec<_> = values
            .iter()
            .flat_map(|&(b, e)| [(b, 1i32), (e+1 , -1 )])
            .collect();
        pairs.sort_unstable();
        let mut sweep_line = vec![0usize;n];
        let mut counter = 0;
        for (index,_type) in pairs {
            counter += _type;
            if index < n {
                sweep_line[index] = counter as usize;
            }
        }
        //fill empty spaces with previous value (if the cell is empty, then it means that we didn't increment the previous value)
        let mut last_encountered = 0; 
        sweep_line.iter_mut().for_each(|elem| {
            if *elem == 0{ *elem = last_encountered; }
            else { last_encountered = *elem; } 
        });
        let mut array = vec![HashSet::new(); tree_size];
        SegmentTree::build_util (&mut array, 0, input_size - 1, &sweep_line, 0, *sweep_line.last().unwrap());
        Self { 
            array,
            end: input_size - 1
        }
        
    } 
    fn build_util(array: &mut Vec<HashSet<usize>>, start: usize, end: usize, values: &Vec<usize>, i: usize, dummy: usize)  -> HashSet<usize> {
        if start == end {
            //fill extra empty space with last value of sweep line
            let value = if end < values.len() { values[end] } else { dummy };
            array[i].insert(value);
            return array[i].clone();
        }
        let mid = start + ( end - start) / 2;
        let left = SegmentTree::build_util(array,start, mid, values, i*2+1, dummy);
        let right = SegmentTree::build_util(array,mid + 1, end, values, i*2 + 2, dummy);
        array[i].extend(left);
        array[i].extend(right);
        array[i].clone()
    }
    pub fn is_there(&mut self, start: usize, end: usize, k : usize) -> usize {
        self.rq(0,0, self.end  , start, end, k)
    }
    fn rq(&mut self, tree_index: usize, left: usize, right: usize, start: usize, end: usize, k : usize) -> usize{

        if  start > end {
            //return default value when query falls outside of array
            return 0;
        }
        //complete overlap
        if start == left && end == right {
            let res = &self.array[tree_index];
            return match res.get(&k)  {
                Some(_) => 1,
                None => 0,
            } 
        }
        //partial overlap cases
        let mid = left + (right - left) / 2;
        let mut max = 0;
        max = max.max(self.rq(2*tree_index+1, left, mid, start, end.min(mid),k));
        max = max.max(self.rq(2*tree_index+2, mid+1, right, start.max(mid+1), end,k));
        max
    }
}

fn main() {
    
}

#[cfg(test)]
mod tests {
    use crate::{test_cases_reader::TestCaseReader, SegmentTree};
    use std::fmt::format;

    #[test]
    fn professor_test_cases(){
        for key in 0..8{
            println!("----------STARTING TEST CASE {}------",key);
            let mut input_reader = TestCaseReader::create(format(format_args!("src/Testset_handson2_p2/input{}.txt",key)));
            let (n,m) = input_reader.read_first_line();
            let mut vec : Vec<(usize,usize)> = Vec::new();
            input_reader.read_lines(&mut |_1,_2,_3,_4| {
                vec.push((_1 as usize,_2 as usize));
            }, n as usize);
            let mut operations = Vec::<(usize,usize,usize)>::new();
            input_reader.read_lines(&mut |_1,_2,_3,_4| {
                operations.push((_1 as usize,_2 as usize,_3 as usize));
            }, m as usize);
            let mut output_reader = TestCaseReader::create(format(format_args!("src/Testset_handson2_p2/output{key}.txt")));
            let mut outputs = Vec::<usize>::new();
            output_reader.read_lines(&mut |_1,_2,_3,_4| {
                outputs.push(_1 as usize);
            }, usize::MAX);
            println!("\nExpected output for case {} {:?}\n", key, outputs); 
            let mut tree = SegmentTree::build(vec, n as usize);
    
            let mut iter = 0;
            operations.iter().for_each(|&(start,end, k)| {
                assert_eq!(tree.is_there(start, end, k), outputs[iter]);
                iter += 1;
            });
            println!("----------END OF TEST CASE {}------",key);
        }
        
    }
}
