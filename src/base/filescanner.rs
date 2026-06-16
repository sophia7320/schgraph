#![allow(unused_variables)]
#![allow(dead_code)]
use core::panic;
use std::{fs, str::FromStr};

use crate::base::app::Place;

#[derive(Debug)]
pub struct FileScanner {
    content: Vec<String>,
    pos: usize,
}

impl FileScanner {
    pub fn from_file_path(file_path: &str) -> Self {
        let content = fs::read_to_string(file_path)
            .expect("faile to read content from file")
            .split_whitespace()
            .map(String::from)
            .collect();
        Self { content, pos: 0 }
    }

    pub fn iter(&self) -> ScannerIter<'_> {
        ScannerIter {
            scanner: self,
            pos: 0,
        }
    }
}

pub struct ScannerIter<'a> {
    scanner: &'a FileScanner,
    pos: usize,
}

impl<'a> Iterator for ScannerIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.scanner.content.len() {
            let token = &self.scanner.content[self.pos];
            self.pos += 1;
            Some(token.as_str())
        } else {
            None
        }
    }
}

impl<'a> ScannerIter<'a> {
    //todo impl next_chunk
    fn next_chunk(&mut self, n: usize) -> Option<Vec<&'a str>> {
        let mut chunk = Vec::with_capacity(n);
        for _ in 0..n {
            if let Some(val) = self.next() {
                chunk.push(val);
            } else {
                break;
            }
        }

        if chunk.is_empty() { None } else { Some(chunk) }
    }

    fn next_chunk_edge(&mut self) -> Option<(String, String, u64)> {
        self.next_chunk(3).map(|x| match x.as_slice() {
            [a, b, c] => (
                String::from_str(a).unwrap(),
                String::from_str(b).unwrap(),
                c.parse::<u64>().unwrap(),
            ),

            _ => panic!("invalid chunk size"),
        })
    }

    fn next_desc(&mut self) -> Option<(String, String)> {
        self.next_chunk(2).map(|x| match x.as_slice() {
            [a, b] => (String::from_str(a).unwrap(), String::from_str(b).unwrap()),

            _ => panic!("invalid chunk size"),
        })
    }

    pub(super) fn get_places(&mut self) -> Vec<Place> {
        let n = self
            .next()
            .unwrap()
            .parse()
            .expect("unable to parse the amount of places");

        let mut res = Vec::with_capacity(n);

        for idx in 0..n {
            let (id, (name, desc)) = (idx, self.next_desc().unwrap());

            res.push(Place::new(id, name, desc));
        }

        res
    }

    pub(super) fn get_raw_edges(&mut self) -> Vec<(String, String, u64)> {
        let n: usize = self
            .next()
            .unwrap()
            .parse()
            .expect("unable to parse the amount of edge");

        let mut res = Vec::with_capacity(n);

        for _ in 0..n {
            res.push(self.next_chunk_edge().unwrap());
        }

        eprintln!("{:#?}", res);

        res
    }
}

#[cfg(test)]
mod test {
    use crate::base::filescanner::FileScanner;

    #[test]
    fn scanfile() {
        let file_path = "tests/fixtures/test_filescanner.txt";
        let scanner = FileScanner::from_file_path(file_path);

        let mut fileiter = scanner.iter();

        let first = fileiter.next().unwrap().parse::<u64>().unwrap();
        assert_eq!(first, 3);

        let mut next_tuple = || -> (u64, u64, u64) {
            fileiter
                .next_chunk(3)
                .map(|x| match x.as_slice() {
                    [a, b, c] => (
                        a.parse::<u64>().unwrap(),
                        b.parse::<u64>().unwrap(),
                        c.parse::<u64>().unwrap(),
                    ),

                    _ => panic!("invalid chunk size"),
                })
                .unwrap()
        };

        assert_eq!(next_tuple(), (1, 2, 3));
        assert_eq!(next_tuple(), (4, 5, 6));
        assert_eq!(next_tuple(), (7, 8, 9));
    }
}
