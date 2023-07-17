use std::cmp::Reverse;

pub struct Graph {
    pub edges: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(edges: Vec<Vec<usize>>) -> Self {
        Self { edges }
    }
    pub fn from_pair_of_nodes(node_size: usize, edges : Vec<(usize, usize)>) -> Self {
        let mut arrows = vec![vec![]; node_size];
        for &(a, b) in &edges {
            arrows[a].push(b);
            arrows[b].push(a);
        }
        Self { edges: arrows }
    }
    pub fn dijkstra(self: &Self, start: usize) -> Vec<usize> {
        assert!(start < self.edges.len());
        let mut max = 0;
        // dijkstra using binary heap
        let mut min_len: Vec<usize> = vec![std::usize::MAX; self.edges.len()];
        let mut heap = std::collections::BinaryHeap::new();
        heap.push((Reverse(0), start));
        while let Some((len, node)) = heap.pop() {
            if len.0 > min_len[node] {
                continue;
            }
            min_len[node] = len.0;
            max = std::cmp::max(max, len.0);
            for &next in &self.edges[node] {
                if len.0 + 1 < min_len[next] {
                    heap.push((Reverse(len.0 + 1), next));
                }
            }
        }
        min_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
