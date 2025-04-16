use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

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
    // Bron-Kerbosch algorithm with pivoting to find only maximum clique
    fn bron_kerbosch_pivot(
        clique: &FxHashSet<usize>,
        candidates: &mut FxHashSet<usize>,
        processed: &mut FxHashSet<usize>,
        adjacency_list: &FxHashMap<usize, FxHashSet<usize>>,
        maximum_clique: &mut Vec<usize>,
    ) {
        // Candidates and processed empty means no more nodes
        if candidates.is_empty() && processed.is_empty() {
            if clique.len() > maximum_clique.len() {
                // But clique size record means new maximum
                *maximum_clique = clique.iter().copied().collect();
            }

            return;
        }

        // Choose pivot node
        let pivot = candidates
            .union(processed)
            .max_by_key(|&node| adjacency_list.get(node).map_or(0, FxHashSet::len))
            .copied();

        if let Some(pivot) = pivot {
            let candidates_without_pivot_neighbors = candidates
                .difference(&adjacency_list.get(&pivot).cloned().unwrap_or_default())
                .copied()
                .collect_vec();

            for node in candidates_without_pivot_neighbors {
                // New clique includes node from candidate
                let mut new_clique = clique.clone();
                new_clique.insert(node);

                // New candidates is the intersection of candidates and neighbors
                let node_neighbors = &adjacency_list.get(&node).cloned().unwrap_or_default();
                let mut new_candidates = candidates.intersection(node_neighbors).copied().collect();

                // New processed is the intersection of processed and neighbors
                let mut new_processed = processed.intersection(node_neighbors).copied().collect();

                // Recursive call
                bron_kerbosch_pivot(
                    &new_clique,
                    &mut new_candidates,
                    &mut new_processed,
                    adjacency_list,
                    maximum_clique,
                );

                // Node got processed
                candidates.remove(&node);
                processed.insert(node);
            }
        }
    }

    let (adjacency_list, _) = lan_party_graph(input);

    let mut maximum_clique = Vec::with_capacity(10);

    bron_kerbosch_pivot(
        &FxHashSet::default(),
        &mut adjacency_list.keys().copied().collect(),
        &mut FxHashSet::default(),
        &adjacency_list,
        &mut maximum_clique,
    );

    // Retrieve password from maximum clique
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

fn lan_party_graph(input: &str) -> (FxHashMap<usize, FxHashSet<usize>>, Vec<[bool; 676]>) {
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
            .or_insert_with(|| FxHashSet::with_capacity(16))
            .insert(to);
        adjacency_list
            .entry(to)
            .or_insert_with(|| FxHashSet::with_capacity(16))
            .insert(from);

        adjacency_matrix[from][to] = true;
        adjacency_matrix[to][from] = true;
    }

    (adjacency_list, adjacency_matrix)
}
