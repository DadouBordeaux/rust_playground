#[cfg(test)]
mod test;

use std::ops::{ RangeInclusive};

const POSSIBLE_NEIGHBOUR_NUMBER: RangeInclusive<u8> = 0..=8;

struct ConwayGameOfLife {
    /// The current state of the game
    state: Vec<Vec<Cell>>,
}

impl ConwayGameOfLife {
    fn new(state: Vec<Vec<Cell>>) -> Self {
        ConwayGameOfLife { state }
    }

    fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.state[y][x]
    }
}

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
                cell.would_be_alive(
                    neighbourhood_occupancy(
                        neighbour_number(count_neighbours(&game, x, y) as u8).unwrap()
                    )
                )
            )
            .collect::<Vec<Cell>>()
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
            match game.get_cell(x, y) {
                Cell::Alive => acc + 1,
                Cell::Dead => acc
            }
        })
}

fn neighbourhood_occupancy(neighbours_number: u8) -> NeighbourhoodOccupancy {
    use NeighbourhoodOccupancy::*;
    match neighbours_number {
        0..=1 => Underpopulated,
        2 => Survivable,
        3 => Suitable,
        _ => Overpopulated
    }
}

pub enum Cell {
    Alive,
    Dead
}

impl Cell {
    pub fn would_be_alive(&self, neighbour_state: NeighbourhoodOccupancy) -> Self {
        use NeighbourhoodOccupancy::*;
        match neighbour_state {
            Underpopulated | Overpopulated => Cell::Dead,
            Survivable => match &self {
                &Cell::Alive => Cell::Alive,
                &Cell::Dead => Cell::Dead,
            },
            Suitable => Cell::Alive
        }
    }
}
