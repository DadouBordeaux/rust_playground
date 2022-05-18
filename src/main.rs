

/// Should construct a conway game of life
fn main() {}

#[derive(Clone, Debug)]
struct ConwayGameOfLife {
    /// The current state of the game
    state: Vec<Vec<bool>>,
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

impl ConwayGameOfLife {
    fn new(state: Vec<Vec<bool>>) -> Self {
        ConwayGameOfLife { state }
    }

    fn get_cell(&self, x: usize, y: usize) -> bool {
        //TODO fix test test_count_from_zero_to_height_neighbours_with_cell_in_the_top_left
        self.state[y][x]
    }

    fn count_neighbours(&self, x: usize, y: usize) -> usize {
        neighbour_position(x, y)
            .into_iter()
            .fold(0, |acc, (x, y)| {
                if self.get_cell(x, y) {
                    acc + 1
                } else {
                    acc
                }
            })
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
                let neighbours_number = game.count_neighbours(x, y);

                let is_underpopulation = neighbours_number < 2;
                let is_overpopulation = neighbours_number > 3;

                let is_surviving = *cell && !is_underpopulation && !is_overpopulation;
                let is_reproducing = neighbours_number == 3;

                is_reproducing || is_surviving
            })
            .collect::<Vec<bool>>()
    });

    ConwayGameOfLife::new(new_state.collect())
}

#[test]
fn test_count_from_zero_to_height_neighbours_with_cell_in_the_middle() {
    let all_test = vec![
        vec![
            vec![false, false, false],
            vec![false, true, false],
            vec![false, false, false],
        ],
        vec![
            vec![true, false, false],
            vec![false, true, false],
            vec![false, false, false],
        ],
        vec![
            vec![true, false, false],
            vec![true, true, false],
            vec![false, false, false],
        ],
        vec![
            vec![true, false, false],
            vec![true, true, false],
            vec![true, false, false],
        ],
        vec![
            vec![true, true, false],
            vec![true, true, false],
            vec![true, false, false],
        ],
        vec![
            vec![true, true, false],
            vec![true, true, false],
            vec![true, true, false],
        ],
        vec![
            vec![true, true, true],
            vec![true, true, false],
            vec![true, true, false],
        ],
        vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, false],
        ],
        vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
        ]
    ];

    for (index, matrix) in all_test.into_iter().enumerate() {
        let game = ConwayGameOfLife::new(matrix);
        eprintln!("index {}", index);
        assert_eq!(game.count_neighbours(1, 1), index)
    }
}

#[test]
fn test_count_from_zero_to_height_neighbours_with_cell_in_the_top_left() {
    let all_test = vec![
        vec![
            vec![true, false, false],
            vec![false, false, false],
            vec![false, false, false],
        ],
    ];

    for (index, matrix) in all_test.into_iter().enumerate() {
        let game = ConwayGameOfLife::new(matrix);
        eprintln!("index {}", index);
        assert_eq!(game.count_neighbours(0, 0), index)
    }
}
