use std::fs::File;
use std::io::{ErrorKind, Write};

use petgraph::graphmap::DiGraphMap;
use serde::Deserialize;

pub fn compute_next(number: u64) -> u64 {
    if number <= 1 {
        return 1;
    };
    if number % 2 == 0 {
        number / 2
    } else {
        3 * number + 1
    }
}

pub(crate) trait FileSerialization {
    fn to_file(self, path: &str);
    fn from_file(path: &str) -> Self;
}

pub(crate) trait Collatz {
    fn get_next(&mut self, number: u64) -> u64;
    fn get_path_length(&mut self, number: u64) -> u64;
}

pub struct MemoizedCollatz {
    memory: DiGraphMap<u64, u64>,
}

impl Default for MemoizedCollatz {
    fn default() -> Self {
        MemoizedCollatz {
            memory: DiGraphMap::new(),
        }
    }
}

#[derive(Deserialize, Debug)]
struct Edge {
    a: u64,
    b: u64,
}

impl Collatz for MemoizedCollatz {
    fn get_next(&mut self, number: u64) -> u64 {
        if !self.memory.contains_node(number) {
            self.memory.add_node(number);
        }
        if self
            .memory
            .neighbors_directed(number, petgraph::Outgoing)
            .count()
            > 0
        {
            self.memory
                .neighbors_directed(number, petgraph::Outgoing)
                .next()
                .unwrap()
        } else {
            let next = compute_next(number);
            self.memory.add_node(next);
            self.memory.add_edge(number, next, 1);
            next
        }
    }

    fn get_path_length(&mut self, mut number: u64) -> u64 {
        let mut path_length = 0;
        while number != 1 {
            number = self.get_next(number);
            path_length += 1;
        }
        path_length
    }
}

impl FileSerialization for MemoizedCollatz {
    fn to_file(self, path: &str) {
        let mut out_file = File::create(path).unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create(path).unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
        writeln!(&mut out_file, "a,b").unwrap();
        for (a, b, _weight) in self.memory.all_edges() {
            writeln!(&mut out_file, "{},{}", a, b).unwrap();
        }
    }

    fn from_file(path: &str) -> MemoizedCollatz {
        let mut reader = csv::Reader::from_path(path).unwrap();
        let mut edges: Vec<(u64, u64)> = Vec::new();
        for record in reader.deserialize() {
            let record: Edge = record.unwrap();
            edges.push((record.a, record.b));
        }
        MemoizedCollatz {
            memory: DiGraphMap::from_edges(edges.iter()),
        }
    }
}
