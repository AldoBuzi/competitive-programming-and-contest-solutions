use std::fs::File;
use std::io::{prelude::*, BufReader};

pub struct TestCaseReader {
    reader: BufReader<File>
}

impl TestCaseReader {

    pub fn create (file: String) -> Self {
        let file = File::open(file).unwrap();
        let reader = BufReader::new(file);
        TestCaseReader {
            reader
        }
    }
    //always two elements
    pub fn read_first_line(&mut self) -> (i32,i32) {
        let mut buf = String::new();
        let result = self.reader.read_line(&mut buf);
        match result {
            Ok(_) => {
                buf.pop();
                let res : Vec<i32> = buf.split(" ").map(|elem| { elem.parse::<i32>().unwrap() }).collect();
                if res.len() != 2 {
                    panic!("First line of file contains more or less than two numbers");
                }
                (res[0],res[1])
            },
            Err(_) => panic!("Cannot read first line of file"),
        }
    }
    pub fn read_line (&mut self) -> Vec<i32>{
        let mut buf = String::new();
        match self.reader.read_line(&mut buf) {
            Ok(bytes) => {
                if bytes == 0 {
                    return Vec::<i32>::new(); //EOF
                }
                buf.pop();
                let splitted : Vec<i32> = buf.split(" ").map(|elem| { elem.parse::<i32>().unwrap() }).collect();
                splitted
            },
            Err(_) => panic!("Error occured while trying to read a line"),
        }
    }
    pub fn read_lines<F>(&mut self, f: &mut F, limit: usize)
    where 
        F: FnMut(i32,i32,i32,i32) {
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
                if res.len() < 4 { 0 } else { res[3] }
            );
            read += 1;
        }
        
    }
    
}