use petgraph::graphmap::DiGraphMap;


pub fn compute_next(number: u64) -> u64{
    if number <= 1 {return 1;};
    if number % 2 == 0 {
        number / 2
    } else {
        3 * number + 1
    }
}

pub struct MemoizedCollatz {
    memory: DiGraphMap<u64, u64>
}

impl Default for MemoizedCollatz {
    fn default() -> Self {
        MemoizedCollatz{memory: DiGraphMap::new()}
    }
}

impl MemoizedCollatz {
    pub fn get_next(&mut self, number: u64) -> u64 {
        if !self.memory.contains_node(number) {
            self.memory.add_node(number);
        }
        if self.memory.neighbors_directed(number, petgraph::Outgoing).count() > 0 {
            println!("Cached: {}", {number});
            self.memory.neighbors_directed(number, petgraph::Outgoing).next().unwrap()
        } else {
            println!("Computing: {}", {number});
            let next = compute_next(number);
            self.memory.add_node(next);
            self.memory.add_edge(number, next, 1);
            next
        }
    }

    pub fn get_path_length(&self, number: u64) -> u64 {
        number
    }
}

