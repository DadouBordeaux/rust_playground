#[cfg(test)]
mod test;

use std::ops::{ RangeInclusive};

const POSSIBLE_NEIGHBOUR_NUMBER: RangeInclusive<u8> = 0..=8;

#[derive(Clone, Debug)]
struct ConwayGameOfLife {
    /// The current state of the game
    state: Vec<Vec<bool>>,
}

impl ConwayGameOfLife {
    fn new(state: Vec<Vec<bool>>) -> Self {
        ConwayGameOfLife { state }
    }

    fn get_cell(&self, x: usize, y: usize) -> bool {
        self.state[y][x]
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum NeighbourhoodOccupancy {
    Overpopulated,
    Underpopulated,
    Survivable,
    Suitable
}

fn neighbour_position(x: usize, y: usize) -> Vec<(usize, usize)> {
    if x == 0 && y == 0 {
        vec![(x + 1, y), (x + 1, y + 1), (x, y + 1)]
    } else if x == 0 {
        vec![(x + 1, y), (x + 1, y + 1), (x + 1, y - 1)]
    } else if y == 0 {
        vec![(x - 1, y), (x + 1, y), (x - 1, y + 1), (x + 1, y + 1)]
    } else {
        vec![(x - 1, y - 1), (x - 1, y),
             (x - 1, y + 1),

             (x, y - 1),
             (x, y + 1),

             (x + 1, y - 1),
             (x + 1, y),
             (x + 1, y + 1),
        ]
    }
}

fn generation(game: ConwayGameOfLife) -> ConwayGameOfLife {
    /*
        Any live cell with fewer than two live neighbours dies, as if by underpopulation.
        Any live cell with two or three live neighbours lives on to the next generation.
        Any live cell with more than three live neighbours dies, as if by overpopulation.
        Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    */
    let new_state = game.state.iter().enumerate().map(|(x, row)| {
        row.iter()
            .enumerate()
            .map(|(y, cell)|
                would_be_alive(
                    *cell,
                    neighbourhood_occupancy(
                        neighbour_number(count_neighbours(&game, x, y) as u8).unwrap()
                    )
                )
            )
            .collect::<Vec<bool>>()
    });

    ConwayGameOfLife::new(new_state.collect())
}

pub fn neighbour_number(input: u8) -> Result<u8, &'static str> {
    if POSSIBLE_NEIGHBOUR_NUMBER.contains(&input) {
        Ok(input)
    } else {
        Err("C'est mal")
    }
}

fn count_neighbours(game: &ConwayGameOfLife, x: usize, y: usize) -> usize {
    neighbour_position(x, y)
        .into_iter()
        .fold(0, |acc, (x, y)| {
            if game.get_cell(x, y) {
                acc + 1
            } else {
                acc
            }
        })
}

fn neighbourhood_occupancy(neighbours_number: u8) -> NeighbourhoodOccupancy {
    match neighbours_number {
        0..=1 => NeighbourhoodOccupancy::Underpopulated,
        2 => NeighbourhoodOccupancy::Survivable,
        3 => NeighbourhoodOccupancy::Suitable,
        _ => NeighbourhoodOccupancy::Overpopulated
    }
}

// could be curryfied to separate cell and neighbour state ?
fn would_be_alive(cell: bool, neighbour_state: NeighbourhoodOccupancy) -> bool {
    match neighbour_state {
        NeighbourhoodOccupancy::Underpopulated | NeighbourhoodOccupancy::Overpopulated => false,
        NeighbourhoodOccupancy::Survivable => cell,
        NeighbourhoodOccupancy::Suitable => true,
    }
}
