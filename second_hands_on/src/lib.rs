#[cfg(test)]
mod test_cases_reader;
pub struct SegmentTree {
    array: Vec<i32>,
    lazy: Vec<i32>, 
    end: usize
}

impl SegmentTree {
    pub fn build(values: Vec<i32>) -> Self {
        let input_size = 2usize.pow((values.len() as f32).log2().ceil() as u32);
        let tree_size = 2 * input_size - 1;
        let mut array: Vec<i32> = vec![0i32; tree_size];
        SegmentTree::build_util (&mut array, 0, input_size - 1, &values, 0, *values.last().unwrap());
        Self { 
            array,
            lazy: vec![i32::MAX; tree_size],
            end: input_size - 1
        }
    } 
    fn build_util(array: &mut Vec<i32>, start: usize, end: usize, values: &Vec<i32>, i: usize, last_value: i32)  -> i32 {
        if start == end {
            let value = if end < values.len() { values[end] } else { last_value };
            array[i] = value;
            return value;
        }
        let mid = start + ( end - start) / 2;
        array[i] = SegmentTree::build_util(array,start, mid, values, i*2+1, last_value).max(
            SegmentTree::build_util(array,mid + 1, end, values, i*2 + 2, last_value)
        );
        array[i]
    }

    pub fn update(&mut self, start:usize, end: usize, new_value: i32){
        self.updt(0, self.end, start-1, end-1,0, new_value);
    }
    fn updt(&mut self, left: usize, right: usize, start: usize, end: usize, i: usize, new_value: i32) {

        if start > end {
            return;
        }

        if start == left && end == right {
            if new_value < self.array[i] {
                self.lazy[i] = new_value;
            }
            self.array[i] = self.array[i].min(new_value);
        }
        else {
            self.push(i);
            let mid = left + (right - left) / 2;
            
            self.updt(left, mid, start, end.min(mid), 2* i + 1, new_value);
            self.updt(mid+1, right,start.max(mid+1), end, 2 * i + 2, new_value);   
            
            self.array[i] = self.array[2*i+1].max(self.array[2*i+2]);
        }

    }
    fn push(&mut self, index : usize) {
        if self.array[2 * index  + 1] > self.lazy[index] {
            self.lazy[2*index + 1] = self.lazy[index];
        }
        if self.array[2 * index  + 2] > self.lazy[index] {
            self.lazy[2*index + 2] = self.lazy[index];
        }
        self.array[2 * index  + 1] = self.array[2 * index  + 1].min(self.lazy[index]);
        self.array[2 * index  + 2] = self.array[2 * index  + 2].min(self.lazy[index]);
        

        self.lazy[index] = i32::MAX;
    }
    pub fn max(&mut self, start: usize, end: usize) -> i32 {
        self.rq(0,0, self.end  , start-1, end-1)
    }
    fn rq(&mut self, tree_index: usize, left: usize, right: usize, start: usize, end: usize) -> i32{
        if  start > end {
            //return default value when query falls outside of array
            return i32::MIN;
        }
        //complete overlap
        if start == left && end == right {
            return self.array[tree_index];
        }
        self.push(tree_index);
        //partial overlap cases
        let mid = left + (right - left) / 2;
        let mut max = i32::MIN;
        max = max.max(self.rq(2*tree_index+1, left, mid, start, end.min(mid)));
        max = max.max(self.rq(2*tree_index+2, mid+1, right, start.max(mid+1), end));
        max
    }
}

#[cfg(test)]
mod tests {
    use crate::test_cases_reader::TestCaseReader;
    use crate::SegmentTree;
    use std::fmt::format;


    #[test]
    fn test_cases(){
        for key in 0..11{
            println!("----------STARTING TEST CASE {}------",key);
            let mut input_reader = TestCaseReader::create(format(format_args!("src/Testset_handson2_p1/input{}.txt",key)));
            
            let (_,m) = input_reader.read_first_line();
            let vec : Vec<i32> = input_reader.read_line();
            let mut operations = Vec::<(i32,i32,i32,i32)>::new();
            input_reader.read_lines(&mut |_1,_2,_3,_4| {
                operations.push((_1,_2,_3,_4));
            }, m as usize);
            let mut output_reader = TestCaseReader::create(format(format_args!("src/Testset_handson2_p1/output{key}.txt")));
            let mut outputs = Vec::<i32>::new();
            output_reader.read_lines(&mut |_1,_2,_3,_4| {
                outputs.push(_1);
            }, usize::MAX);
            println!("\nExpected output for case {} {:?}\n", key, outputs); 
            let mut tree = SegmentTree::build(vec);
    
            let mut iter = 0;
            operations.iter().for_each(|&(op_type,start,end,extra)| {
                if op_type == 1 {
                    assert_eq!(tree.max(start as usize, end as usize), outputs[iter]);
                    iter +=1;
                }
                else {
                    tree.update(start as usize, end as usize, extra);
                }
            });
            println!("----------END OF TEST CASE {}------",key);
        }
        
    }
}
