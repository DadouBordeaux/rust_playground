#[cfg(test)]
mod test;


#[derive(PartialEq, Eq, Debug)]
enum NeighbourhoodOccupancy {
    Overpopulated,
    Underpopulated,
    Averagepopulated
}

use std::ops::{ RangeInclusive};

const POSSIBLE_NEIGHBOUR_NUMBER: RangeInclusive<u8> = 0..=8;

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
        //TODO fix test test_count_from_zero_to_height_neighbours_with_cell_in_the_top_left
        self.state[y][x]
    }
}

fn conway(game: ConwayGameOfLife) -> ConwayGameOfLife {
    /*
        Any live cell with fewer than two live neighbours dies, as if by underpopulation.
        Any live cell with two or three live neighbours lives on to the next generation.
        Any live cell with more than three live neighbours dies, as if by overpopulation.
        Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    */
    let new_state = game.state.iter().enumerate().map(|(x, row)| {
        row.iter()
            .enumerate()
            .map(|(y, cell)| {
                let neighbours_number = count_neighbours(&game, x, y) as u8;

                // let is_underpopulation = neighbours_number < 2;
                // let is_overpopulation = neighbours_number > 3;
                //
                // let is_reproducing = neighbours_number == 3;
                // let is_surviving = *cell && !is_underpopulation && !is_overpopulation;
                //
                // is_reproducing || is_surviving
                *cell && would_survive(neighbourhood_occupancy(neighbours_number))

            })
            .collect::<Vec<bool>>()
    });

    ConwayGameOfLife::new(new_state.collect())
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
        2..=3 => NeighbourhoodOccupancy::Averagepopulated,
        _ => NeighbourhoodOccupancy::Overpopulated
    }
}

fn would_survive(neighbour_state: NeighbourhoodOccupancy) -> bool {
    /*! matches!(
              neighbour_state,
              NeighbourhoodOccupancy::Underpopulated | NeighbourhoodOccupancy::Overpopulated
       )*/
    match neighbour_state {
        NeighbourhoodOccupancy::Underpopulated | NeighbourhoodOccupancy::Overpopulated => false,
        NeighbourhoodOccupancy::Averagepopulated => true,
    }
}
