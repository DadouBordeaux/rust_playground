 pub use super::*;

#[test]
fn test_count_from_zero_to_height_neighbours_with_cell_in_the_top_left() {
    let game = ConwayGameOfLife::default();
    let game = game.with_alive_cell((0, 0));
    assert_eq!(count_neighbours(&game, 1, 1), 1)
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
            ConwayGameOfLife::default()
                .with_alive_cell((1, 1))
        ),
        (
            1,
            ConwayGameOfLife::default()
                .with_alive_cell((1, 1))
                .with_alive_cell((0, 0))
        ),
        (
            2,
            ConwayGameOfLife::default()
                .with_alive_cell((0, 0))
                .with_alive_cell((0, 1)).with_alive_cell((1, 1))
        ),
        (
            3,
            ConwayGameOfLife::default()
                .with_alive_cell((0, 0))
                .with_alive_cell((0, 1)).with_alive_cell((1, 1))
                .with_alive_cell((0, 2))
        ),
        (
            4,
            ConwayGameOfLife::default()
                .with_alive_cell((0, 0)).with_alive_cell((1, 0))
                .with_alive_cell((0, 1)).with_alive_cell((1, 1))
                .with_alive_cell((0, 2))

        ),
        (
            5,
            ConwayGameOfLife::default()
                .with_alive_cell((0, 0)).with_alive_cell((1, 0))
                .with_alive_cell((0, 1)).with_alive_cell((1, 1))
                .with_alive_cell((0, 2)).with_alive_cell((1, 2))
        ),
        (
            6,
            ConwayGameOfLife::default()
                .with_alive_cell((0, 0)).with_alive_cell((1, 0)).with_alive_cell((2, 0))
                .with_alive_cell((0, 1)).with_alive_cell((1, 1))
                .with_alive_cell((0, 2)).with_alive_cell((1, 2))
        ),
        (
            7,
            ConwayGameOfLife::default()
                .with_alive_cell((0, 0)).with_alive_cell((1, 0)).with_alive_cell((2, 0))
                .with_alive_cell((0, 1)).with_alive_cell((1, 1)).with_alive_cell((2, 1))
                .with_alive_cell((0, 2)).with_alive_cell((1, 2))
        ),
        (
            8,
            ConwayGameOfLife::default()
                .with_alive_cell((0, 0)).with_alive_cell((1, 0)).with_alive_cell((2, 0))
                .with_alive_cell((0, 1)).with_alive_cell((1, 1)).with_alive_cell((2, 1))
                .with_alive_cell((0, 2)).with_alive_cell((1, 2)).with_alive_cell((2, 2))
        ),
    ];

    for (expected_neighbours_number, game) in all_test.into_iter() {

        assert_eq!(count_neighbours(&game, 1, 1), expected_neighbours_number)
    }
}
