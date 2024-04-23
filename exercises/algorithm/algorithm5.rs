use std::collections::VecDeque;

struct Graph {
    adj: Vec<Vec<usize>>, // adjacency list representation of the graph
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n], // initialize each vertex with an empty list of neighbors
        }
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); // add dest to the list of src's neighbors
        self.adj[dest].push(src); // add src to the list of dest's neighbors, because it's an undirected graph
    }

    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        let mut visit_order = vec![];
        let mut visited = vec![false; self.adj.len()]; // track visited vertices
        let mut queue = VecDeque::new();

        // Start BFS from the starting vertex
        visited[start] = true;
        queue.push_back(start);

        while let Some(vertex) = queue.pop_front() {
            visit_order.push(vertex); // record the visit order

            // Visit all the unvisited neighbors
            for &neighbor in &self.adj[vertex] {
                if !visited[neighbor] {
                    visited[neighbor] = true; // mark as visited
                    queue.push_back(neighbor); // enqueue for processing
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

