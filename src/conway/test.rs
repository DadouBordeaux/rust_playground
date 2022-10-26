 pub use super::*;

#[test]
fn test_count_from_zero_to_height_neighbours_with_cell_in_the_top_left() {
    let all_test = vec![
        vec![
            vec![Cell::Alive, Cell::Dead, Cell::Dead],
            vec![Cell::Dead, Cell::Dead, Cell::Dead],
            vec![Cell::Dead, Cell::Dead, Cell::Dead],
        ],
    ];

    for (index, matrix) in all_test.into_iter().enumerate() {
        let game = ConwayGameOfLife::new(matrix);
        eprintln!("index {}", index);
        assert!(matches!(count_neighbours(&game, 0, 0), index)   )
    }
}

#[test]
fn test_overpopulation() {
    for i in 4..=8 {
        let neighbour_count= i;
        let cell_neighbourhood_state = neighbourhood_occupancy(neighbour_count);
        assert!(matches!(cell_neighbourhood_state, NeighbourhoodOccupancy::Overpopulated));
    }
}
#[test]
fn test_underpopulation() {
    for i in 0..=1 {
        let neighbour_count  = i;
        let cell_neighbourhood_state = neighbourhood_occupancy(neighbour_count);
        assert!(matches!(cell_neighbourhood_state, NeighbourhoodOccupancy::Underpopulated) );
    }
}

#[test]
fn test_survivable_population() {
    let neighbour_count  = 2;
    let cell_neighbourhood_state = neighbourhood_occupancy(neighbour_count);
    assert!(matches!(cell_neighbourhood_state, NeighbourhoodOccupancy::Survivable));
}

#[test]
fn test_suitable_population() {
        let neighbour_count= 3;
        let cell_neighbourhood_state = neighbourhood_occupancy(neighbour_count);
        assert!(matches!(cell_neighbourhood_state, NeighbourhoodOccupancy::Suitable));
}


//TODO: use would be alive instead of would_survive or would reproduce
#[test]
fn test_cell_dont_survive_when_underpopulated_population() {
    assert!(
        matches!(
            Cell::Dead.would_be_alive(NeighbourhoodOccupancy::Underpopulated),
            Cell::Dead
        )
    );

    assert!(
         matches!(
            Cell::Alive.would_be_alive( NeighbourhoodOccupancy::Underpopulated),
            Cell::Dead
         )
    );
}

#[test]
fn test_cell_dont_survive_when_overpopulated_population() {
    assert!(
        matches!(
            Cell::Alive.would_be_alive( NeighbourhoodOccupancy::Overpopulated),
            Cell::Dead
        )
    );

    assert!(
        matches!(
            Cell::Dead.would_be_alive( NeighbourhoodOccupancy::Overpopulated),
            Cell::Dead
        )
    );
}

#[test]
fn test_cell_dont_survive_when_survivable() {
    assert!(
        matches!(
            Cell::Alive.would_be_alive(NeighbourhoodOccupancy::Survivable),
            Cell::Alive
        )
    );

    assert!(
        matches!(
            Cell::Dead.would_be_alive(NeighbourhoodOccupancy::Survivable),
            Cell::Dead
        )
    );
}

#[test]
fn test_cell_dont_survive_when_suitable() {
    assert!(
     matches!(
        Cell::Alive.would_be_alive( NeighbourhoodOccupancy::Suitable),
        Cell::Alive
    ));

    assert!(
     matches!(
        Cell::Dead.would_be_alive( NeighbourhoodOccupancy::Suitable),
        Cell::Alive
    ));
}

#[test]
fn test_count_from_zero_to_height_neighbours_with_cell_in_the_middle() {
    let all_test = vec![
        (
            0,
            vec![
                vec![Cell::Dead, Cell::Dead, Cell::Dead],
                vec![Cell::Dead, Cell::Alive, Cell::Dead],
                vec![Cell::Dead, Cell::Dead, Cell::Dead],
            ]
        ),
        (
            1,
            vec![
                vec![Cell::Alive, Cell::Dead, Cell::Dead],
                vec![Cell::Dead, Cell::Alive, Cell::Dead],
                vec![Cell::Dead, Cell::Dead, Cell::Dead],
            ]
        ),
        (
            2,
            vec![
                vec![Cell::Alive, Cell::Dead, Cell::Dead],
                vec![Cell::Alive, Cell::Alive, Cell::Dead],
                vec![Cell::Dead, Cell::Dead, Cell::Dead],
            ]
        ),
        (
            3,
            vec![
                vec![Cell::Alive, Cell::Dead, Cell::Dead],
                vec![Cell::Alive, Cell::Alive, Cell::Dead],
                vec![Cell::Alive, Cell::Dead, Cell::Dead],
            ],
        ),
        (
            4,
            vec![
                vec![Cell::Alive, Cell::Alive, Cell::Dead],
                vec![Cell::Alive, Cell::Alive, Cell::Dead],
                vec![Cell::Alive, Cell::Dead, Cell::Dead],
            ]
        ),
        (
            5,
            vec![
                vec![Cell::Alive, Cell::Alive, Cell::Dead],
                vec![Cell::Alive, Cell::Alive, Cell::Dead],
                vec![Cell::Alive, Cell::Alive, Cell::Dead],
            ]
        ),
        (
            6,
            vec![
                vec![Cell::Alive, Cell::Alive, Cell::Alive],
                vec![Cell::Alive, Cell::Alive, Cell::Dead],
                vec![Cell::Alive, Cell::Alive, Cell::Dead],
            ]
        ),
        (
            7,
            vec![
                vec![Cell::Alive, Cell::Alive, Cell::Alive],
                vec![Cell::Alive, Cell::Alive, Cell::Alive],
                vec![Cell::Alive, Cell::Alive, Cell::Dead],
            ],
        ),
        (
            8,
            vec![
                vec![Cell::Alive, Cell::Alive, Cell::Alive],
                vec![Cell::Alive, Cell::Alive, Cell::Alive],
                vec![Cell::Alive, Cell::Alive, Cell::Alive],
            ]
        )
    ];

    for (expected_neighbours_number, matrix) in all_test.into_iter() {
        let game = ConwayGameOfLife::new(matrix);
        assert_eq!(count_neighbours(&game, 1, 1), expected_neighbours_number)
    }
}
