use itertools::Itertools;
use rustc_hash::FxHashMap;

use crate::random_utils::FxHashWithCapacity;

// ------------------------------------------------------------------------------------------------
// Exports

pub fn graph_triangles_count(input: &str) -> usize {
    let (adjacency_list, adjacency_matrix) = lan_party_graph(input);

    let mut seen_nodes = [false; 676];
    let mut triangles_count = 0;

    // For each node starting with 't'
    for node1 in 494..520 {
        if let Some(neighbors) = adjacency_list.get(&node1) {
            seen_nodes[node1] = true;

            // Check whether any two neighbors are also connected
            for (&node2, &node3) in neighbors.iter().tuple_combinations() {
                if !seen_nodes[node2] && !seen_nodes[node3] && adjacency_matrix[node2][node3] {
                    triangles_count += 1;
                }
            }
        }
    }

    triangles_count
}

pub fn maximum_clique_password(input: &str) -> String {
    let (adjacency_list, adjacency_matrix) = lan_party_graph(input);

    let mut seen_nodes = [false; 676];
    let (mut clique, mut maximum_clique) = (Vec::new(), Vec::new());

    // Simple but fast greedy approach to finding maximal cliques
    for (start_node, neighbours) in adjacency_list {
        if !seen_nodes[start_node] {
            clique.clear();
            clique.push(start_node);

            // Add neighbors that are connected to every clique node
            for neighbor in neighbours {
                if clique.iter().all(|&node| adjacency_matrix[neighbor][node]) {
                    seen_nodes[neighbor] = true;
                    clique.push(neighbor);
                }
            }

            // LAN party contains a single maximum clique, the biggest of the maximal cliques
            if clique.len() > maximum_clique.len() {
                maximum_clique.clone_from(&clique);
            }
        }
    }

    // Retrieve password from encoded node indices
    maximum_clique.sort_unstable();
    #[allow(clippy::cast_possible_truncation)]
    Itertools::intersperse(
        maximum_clique.into_iter().map(|node| {
            format!(
                "{}{}",
                ((node / 26) as u8 + b'a') as char,
                ((node % 26) as u8 + b'a') as char
            )
        }),
        ",".to_string(),
    )
    .collect()
}

// ------------------------------------------------------------------------------------------------
// Parsers

fn lan_party_graph(input: &str) -> (FxHashMap<usize, Vec<usize>>, Vec<[bool; 676]>) {
    // Arithmetic 2 letter node index encoding
    fn node_to_index(node: &[u8]) -> usize {
        26 * (node[0] - b'a') as usize + (node[1] - b'a') as usize
    }

    let mut adjacency_list = FxHashMap::with_capacity(512);
    let mut adjacency_matrix = vec![[false; 676]; 676];

    for edge in input.as_bytes().chunks(6) {
        let (from, to) = (node_to_index(&edge[..2]), node_to_index(&edge[3..]));

        adjacency_list
            .entry(from)
            .or_insert_with(|| Vec::with_capacity(16))
            .push(to);
        adjacency_list
            .entry(to)
            .or_insert_with(|| Vec::with_capacity(16))
            .push(from);

        adjacency_matrix[from][to] = true;
        adjacency_matrix[to][from] = true;
    }

    (adjacency_list, adjacency_matrix)
}
