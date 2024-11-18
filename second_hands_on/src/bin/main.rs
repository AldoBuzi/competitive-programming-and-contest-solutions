use second_hands_on::Ex1SegmentTree;
use second_hands_on::Ex2SegmentTree;
use std::env;
use std::fmt::format;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[cfg_attr(test, allow(dead_code))]
pub fn main() {
    let args: Vec<_> = env::args().collect();
    //input and output files must be added under /src/Testset_handson2_p1 and /src/Testset_handson2_p2
    //assuming input file are called input0.txt, input1.txt... and output files output0.txt, output1.txt...
    //to test run "cargo run"
    let mut path_ex1 = "src/Testset_handson2_p1/";
    let mut number_of_files_ex1 = 11usize;
    let mut path_ex2 = "src/Testset_handson2_p2/";
    let mut number_of_files_ex2 = 8usize;
    if args.len() == 1 {
        println!("No path has been provided, using default ones");
        println!("To use customized paths provide: base_path_ex1 number_of_files_for_ex1 base_path_ex2 number_of_files_for_ex2");
    }
    else if args.len() == 5 {
        path_ex1 = &args[1];
        number_of_files_ex1 = args[2].parse::<usize>().unwrap();
        path_ex2 = &args[3];
        number_of_files_ex2 = args[4].parse::<usize>().unwrap();
    }
    else {
        panic!("Error related to args passed in command line");
    }
    test_cases_ex1(path_ex1.to_string(),number_of_files_ex1);
    test_cases_ex2(path_ex2.to_string(),number_of_files_ex2);
}

fn test_cases_ex1(path: String, number_of_files: usize) {
    for key in 0..number_of_files {
        println!("----------STARTING TEST CASE {}----------", key);
        let mut input_reader =
            TestCaseReader::create(format(format_args!("{}input{}.txt", path, key)));

        let (_, m) = input_reader.read_first_line();
        let vec: Vec<i32> = input_reader.read_line();
        let mut operations = Vec::<(i32, i32, i32, i32)>::new();
        input_reader.read_lines(
            &mut |_1, _2, _3, _4| {
                operations.push((_1, _2, _3, _4));
            },
            m as usize,
        );
        let mut output_reader = TestCaseReader::create(format(format_args!(
            "src/Testset_handson2_p1/output{key}.txt"
        )));
        let mut outputs = Vec::<i32>::new();
        output_reader.read_lines(
            &mut |_1, _2, _3, _4| {
                outputs.push(_1);
            },
            usize::MAX,
        );
        println!("Expected output for case {} {:?}\n", key, outputs);
        let mut tree = Ex1SegmentTree::build(vec);

        let mut iter = 0;
        operations.iter().for_each(|&(op_type, start, end, extra)| {
            if op_type == 1 {
                assert_eq!(tree.max(start as usize, end as usize), outputs[iter]);
                iter += 1;
            } else {
                tree.update(start as usize, end as usize, extra);
            }
        });
    }
}

fn test_cases_ex2(path: String, number_of_files: usize) {
    for key in 0..number_of_files {
        println!("----------STARTING TEST CASE {}----------", key);
        let mut input_reader =
            TestCaseReader::create(format(format_args!("{}input{}.txt", path, key)));
        let (n, m) = input_reader.read_first_line();
        let mut vec: Vec<(usize, usize)> = Vec::new();
        input_reader.read_lines(
            &mut |_1, _2, _3, _4| {
                vec.push((_1 as usize, _2 as usize));
            },
            n as usize,
        );
        let mut operations = Vec::<(usize, usize, usize)>::new();
        input_reader.read_lines(
            &mut |_1, _2, _3, _4| {
                operations.push((_1 as usize, _2 as usize, _3 as usize));
            },
            m as usize,
        );
        let mut output_reader = TestCaseReader::create(format(format_args!(
            "src/Testset_handson2_p2/output{key}.txt"
        )));
        let mut outputs = Vec::<usize>::new();
        output_reader.read_lines(
            &mut |_1, _2, _3, _4| {
                outputs.push(_1 as usize);
            },
            usize::MAX,
        );
        println!("Expected output for case {} {:?}\n", key, outputs);
        let mut tree = Ex2SegmentTree::build(vec, n as usize);

        let mut iter = 0;
        operations.iter().for_each(|&(start, end, k)| {
            assert_eq!(tree.is_there(start, end, k), outputs[iter]);
            iter += 1;
        });
    }
}

pub struct TestCaseReader {
    reader: BufReader<File>,
}

impl TestCaseReader {
    pub fn create(file: String) -> Self {
        let file = File::open(file).unwrap();
        let reader = BufReader::new(file);
        TestCaseReader { reader }
    }
    //always two elements
    pub fn read_first_line(&mut self) -> (i32, i32) {
        let mut buf = String::new();
        let result = self.reader.read_line(&mut buf);
        match result {
            Ok(_) => {
                buf.pop();
                let res: Vec<i32> = buf
                    .split(" ")
                    .map(|elem| elem.parse::<i32>().unwrap())
                    .collect();
                if res.len() != 2 {
                    panic!("First line of file contains more or less than two numbers");
                }
                (res[0], res[1])
            }
            Err(_) => panic!("Cannot read first line of file"),
        }
    }
    pub fn read_line(&mut self) -> Vec<i32> {
        let mut buf = String::new();
        match self.reader.read_line(&mut buf) {
            Ok(bytes) => {
                if bytes == 0 {
                    return Vec::<i32>::new(); //EOF
                }
                buf.pop();
                let splitted: Vec<i32> = buf
                    .split(" ")
                    .map(|elem| elem.parse::<i32>().unwrap())
                    .collect();
                splitted
            }
            Err(_) => panic!("Error occured while trying to read a line"),
        }
    }
    pub fn read_lines<F>(&mut self, f: &mut F, limit: usize)
    where
        F: FnMut(i32, i32, i32, i32),
    {
        let mut read = 0;
        while read < limit {
            let res = self.read_line();
            if res.is_empty() {
                break;
            }
            if res.len() > 4 {
                panic!("Line contains more than 4 elements ")
            }
            f(
                if res.is_empty() { 0 } else { res[0] },
                if res.len() < 2 { 0 } else { res[1] },
                if res.len() < 3 { 0 } else { res[2] },
                if res.len() < 4 { 0 } else { res[3] },
            );
            read += 1;
        }
    }
}
