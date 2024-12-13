use grid::Grid;
use itertools::Itertools;
use std::collections::VecDeque;

use crate::random_utils::{
    bytes_grid,
    grid_mask::GridMask,
    pos::{GridGet, Pos},
};

// ------------------------------------------------------------------------------------------------
// Exports

pub fn fences_total_cost_perimeter(input: &str) -> usize {
    // Calculate total fences cost using perimeter
    fences_total_cost(
        input,
        |farm_grid, (position, plot), visited_plots, plots_queue, perimeter| {
            position
                .neighbors()
                .filter_map(|neighbor| match farm_grid.pos_get(neighbor) {
                    None => {
                        *perimeter += 1;
                        None
                    }
                    Some(&next) if next != plot => {
                        *perimeter += 1;
                        plots_queue.push_back((neighbor, next));
                        None
                    }
                    Some(_) if !visited_plots.set_true(neighbor) => None,
                    Some(&next) => Some((neighbor, next)),
                })
                .collect_vec()
        },
    )
}

pub fn fences_total_cost_sides(input: &str) -> usize {
    // Calculate total fences cost using sides count
    fences_total_cost(
        input,
        |farm_grid, (position, plot), visited_plots, plots_queue, sides| {
            // Get all neighbors
            let neighbors = position
                .neighbors()
                .filter(|&neighbor| {
                    farm_grid.pos_get(neighbor).is_some_and(|&next| {
                        if next == plot {
                            true
                        } else {
                            plots_queue.push_back((neighbor, next));
                            false
                        }
                    })
                })
                .collect_vec();

            // Check for corners
            *sides += [
                ([Pos::new(1, 0), Pos::new(0, 1)], Pos::new(1, 1)),
                ([Pos::new(0, 1), Pos::new(-1, 0)], Pos::new(-1, 1)),
                ([Pos::new(-1, 0), Pos::new(0, -1)], Pos::new(-1, -1)),
                ([Pos::new(0, -1), Pos::new(1, 0)], Pos::new(1, -1)),
            ]
            .into_iter()
            .map(|(ortho, corner)| (ortho.map(|corner| position + corner), position + corner))
            .filter(
                |([n1, n2], c)| match (neighbors.contains(n1), neighbors.contains(n2)) {
                    (false, false) => true,
                    (true, true) => farm_grid.pos_get(*c).is_some_and(|&next| next != plot),
                    _ => false,
                },
            )
            .count();

            // Return neighbors within same region
            neighbors
                .into_iter()
                .filter(|&neighbor| visited_plots.set_true(neighbor))
                .map(|neighbor| {
                    (
                        neighbor,
                        *farm_grid.pos_get(neighbor).expect("Expected farm plot"),
                    )
                })
                .collect_vec()
        },
    )
}

// ------------------------------------------------------------------------------------------------
// Functions

fn fences_total_cost<Cost>(input: &str, second_cost_parameter_update: Cost) -> usize
where
    Cost: Fn(
        &Grid<u8>,
        (Pos, u8),
        &mut GridMask,
        &mut VecDeque<(Pos, u8)>,
        &mut usize,
    ) -> Vec<(Pos, u8)>,
{
    let farm_grid = bytes_grid(input);

    // Visited plots grid mask, plots queue and fences cost
    let mut visited_plots = GridMask::new(farm_grid.size());
    let mut plots_queue = VecDeque::from([(
        Pos::new(0, 0),
        *farm_grid.get(0, 0).expect("Expected farm plot"),
    )]);
    let mut fences_cost = 0;

    // Process plots queue, skipping visited plots and updating fences cost
    while let Some((position, plot)) = plots_queue.pop_front() {
        if visited_plots[position] {
            continue;
        }

        visited_plots.set_true(position);

        let mut next_plots = vec![(position, plot)];
        let (mut area, mut second_cost_parameter) = (0, 0);

        while !next_plots.is_empty() {
            next_plots = next_plots
                .into_iter()
                .flat_map(|(position, plot)| {
                    area += 1;

                    second_cost_parameter_update(
                        &farm_grid,
                        (position, plot),
                        &mut visited_plots,
                        &mut plots_queue,
                        &mut second_cost_parameter,
                    )
                })
                .collect();
        }

        fences_cost += area * second_cost_parameter;
    }

    fences_cost
}
