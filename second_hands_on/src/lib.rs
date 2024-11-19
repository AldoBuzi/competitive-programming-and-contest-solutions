use std::collections::HashSet;

pub struct Ex1SegmentTree {
    array: Vec<i32>,
    lazy: Vec<i32>,
    end: usize,
}

impl Ex1SegmentTree {
    pub fn build(values: Vec<i32>) -> Self {
        // size of tree is exactly 2 * 2 ** ceil(log_2(n) - 1
        let input_size = 2usize.pow((values.len() as f32).log2().ceil() as u32);
        let tree_size = 2 * input_size - 1;
        let mut array: Vec<i32> = vec![0i32; tree_size];
        Ex1SegmentTree::build_util(
            &mut array,
            0,
            input_size - 1,
            &values,
            0,
            *values.last().unwrap(),
        );
        Self {
            array,
            // initalize with max because the lazy propagation is done by taking the minimum
            lazy: vec![i32::MAX; tree_size],
            end: input_size - 1,
        }
    }
    /*
     * This cost n * logn, since we are scanning all the array and doing logn recursion for the levels
     */
    fn build_util(
        array: &mut Vec<i32>,
        start: usize,
        end: usize,
        values: &Vec<i32>,
        i: usize,
        last_value: i32,
    ) -> i32 {
        if start == end {
            // if the provided array is not a power of two
            // I propagate the last value of the array in the remaining empty cells (since they won't affect the behaviour of the tree)
            // required to have a balanced tree
            let value = if end < values.len() {
                values[end]
            } else {
                last_value
            };
            array[i] = value;
            return value;
        }
        let mid = start + (end - start) / 2;
        // each node contains the maximum value of each interval they are encoding
        array[i] =
            Ex1SegmentTree::build_util(array, start, mid, values, i * 2 + 1, last_value).max(
                Ex1SegmentTree::build_util(array, mid + 1, end, values, i * 2 + 2, last_value),
            );
        array[i]
    }

    pub fn update(&mut self, start: usize, end: usize, new_value: i32) {
        self.updt(0, self.end, start - 1, end - 1, 0, new_value);
    }

    /**
     * logn time to update the tree
     */
    fn updt(
        &mut self,
        left: usize,
        right: usize,
        start: usize,
        end: usize,
        i: usize,
        new_value: i32,
    ) {
        if start > end {
            return;
        }
        // complete overlap, we update only current node and postpone the update to the children by using lazy propagation
        if start == left && end == right {
            // if the new_value is not smaller, than we don't need to propagate it.
            if new_value < self.array[i] {
                self.lazy[i] = new_value;
            }
            // the new value gets saved if it's smaller than the current value
            self.array[i] = self.array[i].min(new_value);
        } else {
            //propagate updates to children
            self.push(i);

            let mid = left + (right - left) / 2;

            //recursive calls both left and right since we are in a partial overlap
            self.updt(left, mid, start, end.min(mid), 2 * i + 1, new_value);
            self.updt(
                mid + 1,
                right,
                start.max(mid + 1),
                end,
                2 * i + 2,
                new_value,
            );
            self.array[i] = self.array[2 * i + 1].max(self.array[2 * i + 2]);
        }
    }
    fn push(&mut self, index: usize) {
        //lazy doesn't get propagated to a child if the lazy value is greater or equal than the current one
        if self.array[2 * index + 1] > self.lazy[index] {
            self.lazy[2 * index + 1] = self.lazy[index];
        }
        if self.array[2 * index + 2] > self.lazy[index] {
            self.lazy[2 * index + 2] = self.lazy[index];
        }
        // if the value is different from max and smaller than the current value, we update the children
        self.array[2 * index + 1] = self.array[2 * index + 1].min(self.lazy[index]);
        self.array[2 * index + 2] = self.array[2 * index + 2].min(self.lazy[index]);

        //the reset value is MAX
        self.lazy[index] = i32::MAX;
    }
    pub fn max(&mut self, start: usize, end: usize) -> i32 {
        self.rq(0, 0, self.end, start - 1, end - 1)
    }
    /**
     * logn time to do a range query
     */
    fn rq(
        &mut self,
        tree_index: usize,
        left: usize,
        right: usize,
        start: usize,
        end: usize,
    ) -> i32 {
        if start > end {
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
        max = max.max(self.rq(2 * tree_index + 1, left, mid, start, end.min(mid)));
        max = max.max(self.rq(2 * tree_index + 2, mid + 1, right, start.max(mid + 1), end));
        max
    }
}

pub struct Ex2SegmentTree {
    array: Vec<HashSet<usize>>,
    end: usize,
}

impl Ex2SegmentTree {
    pub fn build(values: Vec<(usize, usize)>, n: usize) -> Self {
        let input_size = 2usize.pow((n as f32).log2().ceil() as u32);
        let tree_size = 2 * input_size - 1;
        // build sweep line
        //overall the process takes O(n)
        let pairs: Vec<_> = values
            .iter()
            .flat_map(|&(b, e)| [(b, 1i32), (e + 1, -1)])
            .collect();
        let mut sweep_line = vec![0; n];
        for (index, _type) in pairs {
            if index < n {
                sweep_line[index] += _type;
            }
        }
        let mut counter = 0;
        sweep_line.iter_mut().for_each(|elem| {
            counter += *elem;
            *elem = counter
        });
        let mut array = vec![HashSet::new(); tree_size];
        Ex2SegmentTree::build_util(
            &mut array,
            0,
            input_size - 1,
            &sweep_line,
            0,
            *sweep_line.last().unwrap(),
        );
        Self {
            array,
            end: input_size - 1,
        }
    }
    /**
     * Each node uses a hash set containg the values in their respective range
     */
    fn build_util(
        array: &mut Vec<HashSet<usize>>,
        start: usize,
        end: usize,
        values: &Vec<i32>,
        i: usize,
        dummy: i32,
    ) -> HashSet<usize> {
        if start == end {
            //fill extra empty space with last value of sweep line
            let value = if end < values.len() {
                values[end]
            } else {
                dummy
            };
            array[i].insert(value as usize);
            return array[i].clone();
        }
        let mid = start + (end - start) / 2;
        let left = Ex2SegmentTree::build_util(array, start, mid, values, i * 2 + 1, dummy);
        let right = Ex2SegmentTree::build_util(array, mid + 1, end, values, i * 2 + 2, dummy);
        array[i].extend(left);
        array[i].extend(right);
        array[i].clone()
    }
    pub fn is_there(&mut self, start: usize, end: usize, k: usize) -> usize {
        self.rq(0, 0, self.end, start, end, k)
    }

    /**
     * Range query is basically the same as the previous exercise
     */
    fn rq(
        &mut self,
        tree_index: usize,
        left: usize,
        right: usize,
        start: usize,
        end: usize,
        k: usize,
    ) -> usize {
        if start > end {
            //return default value when query falls outside of array
            return 0;
        }
        //complete overlap
        if start == left && end == right {
            let res = &self.array[tree_index];
            return match res.get(&k) {
                Some(_) => 1,
                None => 0,
            };
        }
        //partial overlap cases
        let mid = left + (right - left) / 2;
        let mut max = 0;
        max = max.max(self.rq(2 * tree_index + 1, left, mid, start, end.min(mid), k));
        max = max.max(self.rq(
            2 * tree_index + 2,
            mid + 1,
            right,
            start.max(mid + 1),
            end,
            k,
        ));
        max
    }
}
