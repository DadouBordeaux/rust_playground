pub use super::*;

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
        assert_eq!(count_neighbours(&game, 0, 0), index)
    }
}
/*
    Any live cell with fewer than two live neighbours dies, as if by underpopulation.
    Any live cell with two or three live neighbours lives on to the next generation.
    Any live cell with more than three live neighbours dies, as if by overpopulation.
    Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
*/

pub fn neighbour_number(input: u8) -> Result<u8, &'static str> {
    if POSSIBLE_NEIGHBOUR_NUMBER.contains(&input) {
        Ok(input)
    } else {
        Err("C'est mal")
    }
}

#[test]
fn test_valid_neighbour_number() {
    let out_of_bound_neighbour_number = 9;
    let error = neighbour_number(out_of_bound_neighbour_number);

    assert!(error.is_err())
}

#[test]
fn test_overpopulation() {
    for i in 4..=8 {
        let neighbour_count= neighbour_number(i).unwrap();
        let cell_neighbourhood_state = neighbourhood_occupancy(neighbour_count);
        assert_eq!(cell_neighbourhood_state, NeighbourhoodOccupancy::Overpopulated);
    }

}
#[test]
fn test_underpopulation() {
    for i in 0..=1 {
        let neighbour_count  = i;
        let cell_neighbourhood_state = neighbourhood_occupancy(neighbour_count);
        assert_eq!(cell_neighbourhood_state, NeighbourhoodOccupancy::Underpopulated);
    }

}

#[test]
fn test_average_population() {
    for i in 2..=3 {
        let neighbour_count  = i;
        let cell_neighbourhood_state = neighbourhood_occupancy(neighbour_count);
        assert_eq!(cell_neighbourhood_state, NeighbourhoodOccupancy::Averagepopulated);
    }

}

#[test]
fn test_cell_dont_survive_when_underpopulated_population() {

    let is_surviving = would_survive(NeighbourhoodOccupancy::Underpopulated);
    assert_eq!(is_surviving, false)
}

#[test]
fn test_cell_dont_survive_when_overpopulated_population() {
    let is_surviving = would_survive(NeighbourhoodOccupancy::Overpopulated);

    assert_eq!(is_surviving, false)
}

#[test]
fn test_cell_dont_survive_when_average_population() {
    let is_surviving = would_survive(NeighbourhoodOccupancy::Averagepopulated);

    assert_eq!(is_surviving, true)
}


#[test]
fn test_count_from_zero_to_height_neighbours_with_cell_in_the_middle() {
    let all_test = vec![
        (
            0,
            vec![
                vec![false, false, false],
                vec![false, true, false],
                vec![false, false, false],
            ]
        ),
        (
            1,
            vec![
                vec![true, false, false],
                vec![false, true, false],
                vec![false, false, false],
            ]
        ),
        (
            2,
            vec![
                vec![true, false, false],
                vec![true, true, false],
                vec![false, false, false],
            ]
        ),
        (
            3,
            vec![
                vec![true, false, false],
                vec![true, true, false],
                vec![true, false, false],
            ],
        ),
        (
            4,
            vec![
                vec![true, true, false],
                vec![true, true, false],
                vec![true, false, false],
            ]
        ),
        (
            5,
            vec![
                vec![true, true, false],
                vec![true, true, false],
                vec![true, true, false],
            ]
        ),
        (
            6,
            vec![
                vec![true, true, true],
                vec![true, true, false],
                vec![true, true, false],
            ]
        ),
        (
            7,
            vec![
                vec![true, true, true],
                vec![true, true, true],
                vec![true, true, false],
            ],
        ),
        (
            8,
            vec![
                vec![true, true, true],
                vec![true, true, true],
                vec![true, true, true],
            ]
        )
    ];

    for (expected_neighbours_number, matrix) in all_test.into_iter() {
        let game = ConwayGameOfLife::new(matrix);
        assert_eq!(count_neighbours(&game, 1, 1), expected_neighbours_number)
    }
}
