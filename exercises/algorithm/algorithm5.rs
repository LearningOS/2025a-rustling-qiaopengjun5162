/*
    bfs
    This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src);
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        let n = self.adj.len();
        if n == 0 {
            return vec![];
        }

        // 1. Initialization
        // `visited` array to keep track of discovered nodes
        let mut visited = vec![false; n];

        // `queue` for managing the nodes to explore (FIFO order)
        let mut queue: VecDeque<usize> = VecDeque::new();

        // `visit_order` to store the final result sequence
        let mut visit_order = vec![];

        // 2. Start the BFS
        // Mark the start node as visited and enqueue it
        visited[start] = true;
        queue.push_back(start);

        // 3. Main loop: Continue while there are nodes in the queue
        while let Some(u) = queue.pop_front() {
            // Dequeue the node and record it in the final order
            visit_order.push(u);

            // Explore all neighbors (v) of the current node (u)
            for &v in &self.adj[u] {
                // If the neighbor 'v' has not been visited
                if !visited[v] {
                    // Mark it as visited and enqueue it for later exploration
                    visited[v] = true;
                    queue.push_back(v);
                }
            }
        }
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}
