use std::thread;

#[cfg(test)]
mod test;

use std::ops::{RangeInclusive};

/**
 * Any live cell with fewer than two live neighbours dies, as if by underpopulation.
 * Any live cell with two or three live neighbours lives on to the next generation.
 * Any live cell with more than three live neighbours dies, as if by overpopulation.
 * Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
 */
struct ConwayGameOfLife {
    /// The current state of the game
    ///
    state: Vec<Vec<Cell>>,
}

//TODO test generation
//TODO Display
//TODO Generic Output Display
//TODO Game loop
//TODO Error Management
//TODO Handle negative positions
//TODO game state compression proxy : store only positions of alive cells

impl ConwayGameOfLife {
    // #[cfg(test)]
    // fn new(state: Vec<Vec<Cell>>) -> Self {
    //     ConwayGameOfLife { state }
    // }

    pub fn with_alive_cell(mut self, position: (usize, usize)) -> Self {
        self.state[position.0][position.1] = Cell::Alive;
        self
    }

    pub fn get_cell(&self, x: usize, y: usize) -> &Cell {
        println!("x:{}, y:{}", x, y);
        &self.state[y][x]
    }

    pub fn next(mut self) -> Self {
        let r= self.state.iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .map(|(y, cell)| {
                        cell.would_be_alive(
                            neighbourhood_occupancy(
                                count_neighbours(&self, x, y) as u8
                            )
                        )
                    }).collect()
            }).collect();

        self.state = r;

        self
    }

    fn square(size: usize) -> Self {
        let mut state:Vec<Vec<Cell>>  = Vec::new();
        for _ in 0..size {
            let mut line: Vec<Cell> = Vec::new();
            for _ in 0..size {
                line.push(Cell::Dead);
            }
            state.push(line);
        }
        ConwayGameOfLife { state }
    }
}

impl Default for ConwayGameOfLife {
    fn default() -> Self {
        ConwayGameOfLife {
            state: vec![
                vec![Cell::Dead, Cell::Dead, Cell::Dead],
                vec![Cell::Dead, Cell::Dead, Cell::Dead],
                vec![Cell::Dead, Cell::Dead, Cell::Dead],
            ]
        }
    }
}


enum NeighbourhoodOccupancy {
    Overpopulated,
    Underpopulated,
    Survivable,
    Suitable,
}

fn neighbour_position(x: usize, y: usize) -> Vec<(usize, usize)> {
    if x == 0 && y == 0 {
        vec![(x + 1, y), (x + 1, y + 1), (x, y + 1)]
    } else if x == 0 {
        vec![(x + 1, y), (x + 1, y + 1), (x + 1, y - 1)]
    } else if y == 0 {
        vec![(x - 1, y), (x + 1, y), (x - 1, y + 1), (x + 1, y + 1)]
    } else {
        vec![
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ]
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
        4..=8 => Overpopulated,
        _ => panic!("A cell should never have more than 8 neighbours !")
    }
}

pub enum Cell {
    Alive,
    Dead,
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
