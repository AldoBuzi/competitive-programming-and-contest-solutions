use std::collections::HashSet;

pub struct Ex1SegmentTree {
    array: Vec<i32>,
    lazy: Vec<i32>,
    end: usize,
}

impl Ex1SegmentTree {
    pub fn build(values: Vec<i32>) -> Self {
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
            lazy: vec![i32::MAX; tree_size],
            end: input_size - 1,
        }
    }
    fn build_util(
        array: &mut Vec<i32>,
        start: usize,
        end: usize,
        values: &Vec<i32>,
        i: usize,
        last_value: i32,
    ) -> i32 {
        if start == end {
            let value = if end < values.len() {
                values[end]
            } else {
                last_value
            };
            array[i] = value;
            return value;
        }
        let mid = start + (end - start) / 2;
        array[i] =
            Ex1SegmentTree::build_util(array, start, mid, values, i * 2 + 1, last_value).max(
                Ex1SegmentTree::build_util(array, mid + 1, end, values, i * 2 + 2, last_value),
            );
        array[i]
    }

    pub fn update(&mut self, start: usize, end: usize, new_value: i32) {
        self.updt(0, self.end, start - 1, end - 1, 0, new_value);
    }
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

        if start == left && end == right {
            if new_value < self.array[i] {
                self.lazy[i] = new_value;
            }
            self.array[i] = self.array[i].min(new_value);
        } else {
            self.push(i);
            let mid = left + (right - left) / 2;

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
        if self.array[2 * index + 1] > self.lazy[index] {
            self.lazy[2 * index + 1] = self.lazy[index];
        }
        if self.array[2 * index + 2] > self.lazy[index] {
            self.lazy[2 * index + 2] = self.lazy[index];
        }
        self.array[2 * index + 1] = self.array[2 * index + 1].min(self.lazy[index]);
        self.array[2 * index + 2] = self.array[2 * index + 2].min(self.lazy[index]);

        self.lazy[index] = i32::MAX;
    }
    pub fn max(&mut self, start: usize, end: usize) -> i32 {
        self.rq(0, 0, self.end, start - 1, end - 1)
    }
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
        let mut pairs: Vec<_> = values
            .iter()
            .flat_map(|&(b, e)| [(b, 1i32), (e + 1, -1)])
            .collect();
        pairs.sort_unstable();
        let mut sweep_line = vec![0usize; n];
        let mut counter = 0;
        for (index, _type) in pairs {
            counter += _type;
            if index < n {
                sweep_line[index] = counter as usize;
            }
        }
        //fill empty spaces with previous value (if the cell is empty, then it means that we didn't increment the previous value)
        let mut last_encountered = 0;
        sweep_line.iter_mut().for_each(|elem| {
            if *elem == 0 {
                *elem = last_encountered;
            } else {
                last_encountered = *elem;
            }
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
    fn build_util(
        array: &mut Vec<HashSet<usize>>,
        start: usize,
        end: usize,
        values: &Vec<usize>,
        i: usize,
        dummy: usize,
    ) -> HashSet<usize> {
        if start == end {
            //fill extra empty space with last value of sweep line
            let value = if end < values.len() {
                values[end]
            } else {
                dummy
            };
            array[i].insert(value);
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
