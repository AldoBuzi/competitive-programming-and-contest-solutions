use std::env;
use std::fmt::format;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use hands_on_3::*;

#[cfg_attr(test, allow(dead_code))]
pub fn main() {
    let args: Vec<_> = env::args().collect();
    //input and output files must be added under /src/Testset_handson2_p1 and /src/Testset_handson2_p2
    //assuming input file are called input0.txt, input1.txt... and output files output0.txt, output1.txt...
    //to test run "cargo run"
    let mut path_ex1 = "src/TestSet/";
    let mut number_of_files_ex1 = 5usize;
    let mut path_ex2 = "src/TestSet-2/";
    let mut number_of_files_ex2 = 11usize;
    if args.len() == 1 {
        println!("No path has been provided, using default ones, assuming files are placed under src folder of cargo project");
        println!("To use customized paths provide: base_path_ex1 number_of_files_for_ex1 base_path_ex2 number_of_files_for_ex2");
    } else if args.len() == 5 {
        path_ex1 = &args[1];
        number_of_files_ex1 = args[2].parse::<usize>().unwrap();
        path_ex2 = &args[3];
        number_of_files_ex2 = args[4].parse::<usize>().unwrap();
    } else {
        panic!("Error related to args from cli");
    }
    println!("\n\x1b[32mTEST CASES FOR FIRST EXERCISE\x1b[0m\n");
    test_cases_ex1(path_ex1.to_string(), number_of_files_ex1);
    println!("\n\x1b[32mTEST CASES FOR SECOND EXERCISE\x1b[0m\n");
    test_cases_ex2(path_ex2.to_string(), number_of_files_ex2);
}

/*
 Test Cases for first exercise
*/
fn test_cases_ex1(path: String, number_of_files: usize) {
    for key in 0..number_of_files {
        println!("----------STARTING TEST CASE {}----------", key);
        let mut input_reader =
            TestCaseReader::create(format(format_args!("{}input{}.txt", path, key)));

        let (n, m) = input_reader.read_first_line();
        let mut matrix: Vec<Vec<i32>> = Vec::with_capacity(n as usize);
        input_reader.read_lines(
            &mut |res| {
                matrix.push(res);
            },
            m as usize,
            n as usize,
        );
        let mut output_reader =
            TestCaseReader::create(format(format_args!("{path}output{key}.txt")));
        let mut output = -1;
        output_reader.read_lines(
            &mut |res| {
                output = res[0];
            },
            1,
            1,
        );
        println!("Expected output: {:?}\n", output);
        let res = solution(m as usize, matrix);
        assert_eq!(res, output);
    }
}

/*
 Test Cases for second exercise
*/
fn test_cases_ex2(path: String, number_of_files: usize) {
    for key in 0..number_of_files {
        println!("----------STARTING TEST CASE {}----------", key);
        let mut input_reader =
            TestCaseReader::create(format(format_args!("{}input{}.txt", path, key)));

        let n = input_reader.read_line()[0];
        let mut matrix: Vec<(usize, usize)> = Vec::new();
        input_reader.read_lines(
            &mut |res| {
                matrix.push((res[0] as usize, res[1] as usize));
            },
            2,
            n as usize,
        );
        let mut output_reader =
            TestCaseReader::create(format(format_args!("{path}output{key}.txt")));
        let mut output = -1;
        output_reader.read_lines(
            &mut |res| {
                output = res[0];
            },
            1,
            1,
        );
        println!("Expected output: {:?}\n", output);
        let res = solution2(&mut matrix);
        assert_eq!(res as i32, output, "Testing failed for {} and {}", res, output);
    }
}

/*
 Used to read from input and output files
 -- BAD CODE --
*/

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

    pub fn read_lines<F>(&mut self, f: &mut F, elements_in_line: usize, limit: usize)
    where
        F: FnMut(Vec<i32>),
    {
        let mut read = 0;
        while read < limit {
            let res = self.read_line();
            if res.is_empty() {
                break;
            }
            if res.len() > elements_in_line {
                panic!("Line contains more than {} elements ", elements_in_line)
            }
            f(res);
            read += 1;
        }
    }
}
