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
        let tree = Self { 
                    array: array,
                    lazy: vec![i32::MAX; tree_size],
                    end: input_size - 1
                };
        tree
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

        //println!("Start and and {} - {}", start, end);
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
        //println!("Resulting array {:?}", self.array);
        //println!("Resulting lazy array {:?}", self.lazy);

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

        //println!("My Limit [{},{}] - [{},{}]", left, right, start, end);
        if  start > end {
            //return default value when query falls outside of array
            return i32::MIN;
        }
        //complete overlap
        if start == left && end == right {
            //println!("RETURNED {} {}", self.array[tree_index], tree_index);
            return self.array[tree_index];
        }
        self.push(tree_index);
        //partial overlap cases
        let mid = left + (right - left) / 2;
        let mut max = i32::MIN;
        max = max.max(self.rq(2*tree_index+1, left, mid, start, end.min(mid)));
        max = max.max(self.rq(2*tree_index+2, mid+1, right, start.max(mid+1), end));
        //println!("MAX VALUE {}", max);
        max
    }
}

#[cfg(test)]
mod tests {
    use crate::SegmentTree;
    use std::collections::btree_map::Range;
    use std::fmt::format;
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    #[test]
    fn segment_tree_example() {
        let mut tree = SegmentTree::build( vec![2,3,4,5,6,7,8,9]);

        assert_eq!(tree.max(0, 3), 5);
        assert_eq!(tree.max(0, 7), 9);
        assert_eq!(tree.max(0, 5), 7);
        assert_eq!(tree.max(3, 5), 7);
        assert_eq!(tree.max(3, 7), 9);
        assert_eq!(tree.max(2, 6), 8);

        assert_eq!(tree.max(3, 7), 9);
        assert_eq!(tree.max(1, 6), 8);
        assert_eq!(tree.array[0], 9);
    }

    #[test]
    fn segment_tree_example_2() {
        let mut tree = SegmentTree::build( vec![2,3,4,5,6,7,8,9,10]);

        assert_eq!(tree.array, vec![]);
        assert_eq!(tree.max(0, 3), 5);
        assert_eq!(tree.max(0, 7), 9);
        assert_eq!(tree.max(0, 5), 7);
        assert_eq!(tree.max(3, 5), 7);
        assert_eq!(tree.max(3, 7), 9);
        assert_eq!(tree.max(2, 6), 9);

        assert_eq!(tree.max(3, 7), 9);
        assert_eq!(tree.array[0], 9);
    }
    #[test]
    fn prof_test_case(){
        let mut tree = SegmentTree::build(vec![5,1,4,3,2]);
        //assert_eq!(tree.array, vec![2,1,4,3,2]);
        tree.update(0, 1, 2);
        //assert_eq!(tree.array, vec![2,1,4,3,2]);
        assert_eq!(tree.max(1, 3), 4);
        assert_eq!(tree.max(0, 1), 2);
    }

    #[test]
    fn test_cases(){
        //env::set_var("RUST_BACKTRACE", "1");
        for key in 0..11{
            println!("----------STARTING TEST CASE {}------",key);
            let file = File::open( format(format_args!("src/Testset_handson2_p1/input{}.txt",key))).unwrap();
            let reader = BufReader::new(file);
            let mut n = -1;
            let mut m = -1;
            let mut index = 0;
            let mut index2 = 0;
            let mut vec = Vec::<i32>::new();
            let mut tree : SegmentTree;
            let mut operations = Vec::<(i32,i32,i32,i32)>::new();
            for line in reader.lines() {
                let mut tuple = vec![0;4];
                let mut tuple_index = 0;
                line.unwrap().split(" ").for_each(|x| {
                    if n == -1 {
                        n = x.parse::<i32>().unwrap();
                        vec  = vec![0;n as usize];
                    }
                    else if m == -1 {
                        m = x.parse::<i32>().unwrap();
    
                    } 
                    else if index < n {
                        vec[index as usize] = x.parse::<i32>().unwrap();
                        index += 1;
                    }
                    else if index2 < m {
                        index +=1;
                        if tuple_index < 3 {
                            tuple[tuple_index] = x.parse::<i32>().unwrap();
                            tuple_index += 1;
                        }
                        else if tuple[0] == 0 && tuple_index < 4 {
                            tuple[tuple_index] = x.parse::<i32>().unwrap();
                            tuple_index += 1;
                        }
                        else {
                            tuple_index = 0;
                            index2 += 1;
                        }
                    }
    
                });
                if index > n {
                    operations.push((tuple[0],tuple[1],tuple[2],tuple[3]));
                }
            }
            let file = File::open(format(format_args!("src/Testset_handson2_p1/output{key}.txt"))).unwrap();
            let reader = BufReader::new(file);
            let mut outputs = Vec::<i32>::new();
            for line in reader.lines() {
                outputs.push(line.unwrap().parse::<i32>().unwrap());
            }
            tree = SegmentTree::build(vec);
    
            println!("MyOperations {:?}", operations);
            let mut iter = 0;
            operations.iter().for_each(|&(op_type,start,end,extra)| {
                if op_type == 1 {
                    println!( "Output {}", tree.max(start as usize, end as usize));
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
