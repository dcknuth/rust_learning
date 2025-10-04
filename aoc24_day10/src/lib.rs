use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::{Bfs, VisitMap, Visitable};


fn create_matrix(s: &str) -> Vec<Vec<i32>> {
    let mut m = Vec::new();
    
    for line in s.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as i32);
        }
        m.push(row);
    }

    m
}

/// Converts a 2D matrix of i32 values into an DiGraph where each cell is a node,
/// and edges of weight 1 connect directly adjacent cells (no diagonals).
fn make_graph(matrix: &[Vec<i32>]) ->
    (DiGraph<i32, i32>, Vec<NodeIndex>, Vec<NodeIndex>) {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut graph = DiGraph::<i32, i32>::new();
    let mut trailheads: Vec<NodeIndex> = vec![];
    let mut peaks: Vec<NodeIndex> = vec![];

    // Make starting NodeIndex 2D matrix with a 0 starting value
    let mut node_indices = vec![vec![NodeIndex::new(0); cols]; rows];

    // Add nodes and trailheads
    for r in 0..rows {
        for c in 0..cols {
            let node = graph.add_node(0);
            node_indices[r][c] = node;
            if matrix[r][c] == 0 {
                trailheads.push(node);
            } else if matrix[r][c] == 9 {
                peaks.push(node);
            }

        }
    }

    // Add edges between adjacent nodes only if heights diff by 1
    for r in 0..rows {
        for c in 0..cols {
            let current = node_indices[r][c];
            if r + 1 < rows {
                if matrix[r][c] - matrix[r+1][c] == 1 {
                graph.add_edge(node_indices[r + 1][c], current, 1);
                } else if matrix[r][c] - matrix[r+1][c] == -1 {
                graph.add_edge(current, node_indices[r + 1][c], 1);
                }
            }
            if c + 1 < cols {
                if matrix[r][c] - matrix[r][c+1] == -1 {
                    graph.add_edge(current, node_indices[r][c + 1], 1);
                } else if matrix[r][c] - matrix[r][c+1] == 1 {
                    graph.add_edge(node_indices[r][c + 1], current, 1);
                }
            }
        }
    }

    (graph, trailheads, peaks)
}

fn reachable_peaks(graph: &DiGraph<i32, i32>,
        start: NodeIndex,
        peaks: &[NodeIndex]) -> Vec<NodeIndex> {
    let mut bfs = Bfs::new(graph, start);
    let mut visited = graph.visit_map();
    let mut found = Vec::new();

    while let Some(node) = bfs.next(graph) {
        visited.visit(node);
        if peaks.contains(&node) {
            found.push(node);
            if found.len() == peaks.len() {
                break; // early exit if all goals found
            }
        }
    }

    found
}

pub fn part1(s: &str) -> usize {
    let m = create_matrix(s);
    let (g, trailheads, peaks) = make_graph(&m);
    let mut score = 0;
    for trailhead in trailheads {
        let reachable = reachable_peaks(&g, trailhead, &peaks);
        score += reachable.len();
    }
    
    score
}

#[cfg(test)]
mod test {
    use std::fs;
    use once_cell::sync::Lazy;
    use super::*;

    static S: Lazy<String> = Lazy::new(|| {
        fs::read_to_string("aoc24_day10/test.txt").
            expect("Could not open test file")
    });

    static S1: Lazy<String> = Lazy::new(|| {
        fs::read_to_string("aoc24_day10/test1.txt").
            expect("Could not open test file")
    });

    #[test]
    fn test_small_part1() {
        assert_eq!(part1(&S1), 1);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&S), 36);
    }
}