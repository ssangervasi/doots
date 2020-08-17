use spectral::boolean::BooleanAssertions;
use spectral::{assert_that, asserting};

use doots::game::board::{dot, edge, Board, Dot, DotBox, Edge, WinnerResult};
use doots::players::player::PlayerId;

#[test]
fn it_calculates_dot_count() {
    let board = Board::new(2);
    assert_that!(board.dot_count()).is_equal_to(9);
}

#[test]
fn test_is_free() {
    let mut board = Board::new(2);
    board
        .draw((PlayerId::One, edge((0, 0), (0, 1))))
        .expect("Draw failed!");

    assert_that!(board.is_free(edge((0, 0), (1, 0)))).is_true();
    assert_that!(board.is_free(edge((0, 0), (0, 1)))).is_false();
    assert_that!(board.is_free(edge((0, 1), (0, 0)))).is_false();
}

#[test]
fn test_iter_dots() {
    let board = Board::new(2);
    let dots = board.iter_dots();
    let mut count = 0;

    for (i, dot) in dots.enumerate() {
        println!("{}: {}", i, dot);
        count += 1;
    }
    assert_eq!(board.dot_count(), count)
}

#[test]
fn test_iter_edges_count() {
    let board = Board::new(2);
    let edges = board.iter_edges();
    let mut count = 0;

    for (i, edge) in edges.enumerate() {
        println!("{}: {}", i, edge);
        count += 1;
    }
    assert_eq!(board.edge_count(), count)
}

#[test]
fn test_iter_edges() {
    let board = Board::new(2);
    let mut edges = board.iter_edges();

    assert_eq!(edge((0, 0), (0, 1)), edges.next().unwrap());
    assert_eq!(edge((0, 0), (1, 0)), edges.next().unwrap());

    assert_eq!(edge((0, 1), (0, 2)), edges.next().unwrap());
    assert_eq!(edge((0, 1), (1, 1)), edges.next().unwrap());

    assert_eq!(edge((0, 2), (1, 2)), edges.next().unwrap());

    assert_eq!(edge((1, 0), (1, 1)), edges.next().unwrap());
    assert_eq!(edge((1, 0), (2, 0)), edges.next().unwrap());
}

#[test]
fn test_edge_owner() {
    let mut board = Board::new(2);
    board
        .draw_many(vec![
            (PlayerId::One, edge((1, 1), (1, 2))),
            (PlayerId::Two, edge((1, 1), (2, 1))),
            (PlayerId::One, edge((2, 2), (1, 2))),
            (PlayerId::Two, edge((2, 2), (2, 1))),
        ])
        .expect("Draw failed");

    assert_that!(board.edge_owner(edge((1, 1), (1, 2))).unwrap()).is_equal_to(PlayerId::One);
    assert_that!(board.edge_owner(edge((2, 2), (1, 2))).unwrap()).is_equal_to(PlayerId::One);
    assert_that!(board.edge_owner(edge((1, 1), (2, 1))).unwrap()).is_equal_to(PlayerId::Two);
    assert_that!(board.edge_owner(edge((2, 2), (2, 1))).unwrap()).is_equal_to(PlayerId::Two);

    asserting!("an un-drawn edge is not owned")
        .that(&board.edge_owner(edge((0, 0), (0, 1))))
        .is_equal_to(None);

    asserting!("an out-of-bounds edge is not owned")
        .that(&board.edge_owner(edge((10, 10), (11, 10))))
        .is_equal_to(None);
}

mod test_winner {
    use super::*;

    fn safely_draw_boxes(board: &mut Board, first_owner_id: PlayerId, boxes: &Vec<DotBox>) {
        let mut owner_id = first_owner_id;
        for dotbox in boxes {
            for box_edge in dotbox.edges() {
                if board.is_free(box_edge) {
                    board.draw((owner_id, box_edge)).expect("Draw failed");
                }
                owner_id = if owner_id == PlayerId::One {
                    PlayerId::Two
                } else {
                    PlayerId::One
                }
            }
        }
    }

    #[test]
    fn owner_to_boxes_is_basis_of_winner_logic() {
        let mut board = Board::new(10);
        // Owner: P2
        let p2_boxes = vec![DotBox(dot(0, 0)), DotBox(dot(0, 2)), DotBox(dot(0, 4))];
        safely_draw_boxes(&mut board, PlayerId::One, &p2_boxes);

        // Moves that don't finish a box and end on P2's turn:
        board
            .draw_many(vec![
                (PlayerId::One, edge((3, 0), (4, 0))),
                (PlayerId::Two, edge((4, 0), (5, 0))),
                (PlayerId::One, edge((5, 0), (6, 0))),
            ])
            .expect("Draw failed");

        // Owner: P1
        let p1_boxes = vec![DotBox(dot(5, 5)), DotBox(dot(6, 6))];
        safely_draw_boxes(&mut board, PlayerId::Two, &p1_boxes);

        println!("{}", board.to_string());

        let owner_to_boxes = board.owner_to_boxes();
        assert_that!(*owner_to_boxes.get(&PlayerId::One).unwrap()).is_equal_to(p1_boxes);
        assert_that!(*owner_to_boxes.get(&PlayerId::Two).unwrap()).is_equal_to(p2_boxes);
    }

    #[test]
    fn when_size_one_is_incomplete() {
        let mut board = Board::new(1);
        board
            .draw_many(vec![
                (PlayerId::One, edge((0, 0), (0, 1))),
                (PlayerId::Two, edge((0, 0), (1, 0))),
            ])
            .expect("Draw failed");

        assert_that!(board.winner()).is_equal_to(WinnerResult::None);
    }

    #[test]
    fn when_size_one_is_full() {
        let mut board = Board::new(1);
        safely_draw_boxes(&mut board, PlayerId::One, &vec![DotBox(dot(0, 0))]);
        assert_that!(board.winner()).is_equal_to(WinnerResult::Winner(PlayerId::Two, 1));
    }

    #[test]
    fn when_size_two_is_won_by_one() {
        let mut board = Board::new(2);
        // Example generated by printing out the edges played in a game:
        board
            .draw_many(vec![
                (
                    PlayerId::One,
                    Edge(Dot { row: 0, col: 0 }, Dot { row: 0, col: 1 }),
                ),
                (
                    PlayerId::Two,
                    Edge(Dot { row: 0, col: 0 }, Dot { row: 1, col: 0 }),
                ),
                (
                    PlayerId::One,
                    Edge(Dot { row: 0, col: 1 }, Dot { row: 0, col: 2 }),
                ),
                (
                    PlayerId::Two,
                    Edge(Dot { row: 0, col: 1 }, Dot { row: 1, col: 1 }),
                ),
                (
                    PlayerId::One,
                    Edge(Dot { row: 1, col: 0 }, Dot { row: 1, col: 1 }),
                ),
                (
                    PlayerId::Two,
                    Edge(Dot { row: 0, col: 2 }, Dot { row: 1, col: 2 }),
                ),
                (
                    PlayerId::One,
                    Edge(Dot { row: 1, col: 1 }, Dot { row: 1, col: 2 }),
                ),
                (
                    PlayerId::Two,
                    Edge(Dot { row: 1, col: 0 }, Dot { row: 2, col: 0 }),
                ),
                (
                    PlayerId::One,
                    Edge(Dot { row: 1, col: 2 }, Dot { row: 2, col: 2 }),
                ),
                (
                    PlayerId::Two,
                    Edge(Dot { row: 1, col: 1 }, Dot { row: 2, col: 1 }),
                ),
                (
                    PlayerId::One,
                    Edge(Dot { row: 2, col: 0 }, Dot { row: 2, col: 1 }),
                ),
                (
                    PlayerId::Two,
                    Edge(Dot { row: 2, col: 1 }, Dot { row: 2, col: 2 }),
                ),
            ])
            .expect("Draw failed");

        assert_that!(board.winner()).is_equal_to(WinnerResult::Winner(PlayerId::One, 3));
    }

    #[test]
    fn when_size_two_is_won_by_two() {
        let mut board = Board::new(2);
        // Example generated by printing out the edges played in a game:
        board
            .draw_many(vec![
                (
                    PlayerId::One,
                    Edge(Dot { row: 0, col: 0 }, Dot { row: 1, col: 0 }),
                ),
                (
                    PlayerId::Two,
                    Edge(Dot { row: 0, col: 1 }, Dot { row: 0, col: 2 }),
                ),
                (
                    PlayerId::One,
                    Edge(Dot { row: 0, col: 1 }, Dot { row: 1, col: 1 }),
                ),
                (
                    PlayerId::Two,
                    Edge(Dot { row: 1, col: 0 }, Dot { row: 1, col: 1 }),
                ),
                (
                    PlayerId::One,
                    Edge(Dot { row: 0, col: 2 }, Dot { row: 1, col: 2 }),
                ),
                (
                    PlayerId::Two,
                    Edge(Dot { row: 1, col: 1 }, Dot { row: 1, col: 2 }),
                ),
                (
                    PlayerId::One,
                    Edge(Dot { row: 1, col: 0 }, Dot { row: 2, col: 0 }),
                ),
                (
                    PlayerId::Two,
                    Edge(Dot { row: 1, col: 2 }, Dot { row: 2, col: 2 }),
                ),
                (
                    PlayerId::One,
                    Edge(Dot { row: 1, col: 1 }, Dot { row: 2, col: 1 }),
                ),
                (
                    PlayerId::Two,
                    Edge(Dot { row: 2, col: 0 }, Dot { row: 2, col: 1 }),
                ),
                (
                    PlayerId::One,
                    Edge(Dot { row: 2, col: 1 }, Dot { row: 2, col: 2 }),
                ),
                (
                    PlayerId::Two,
                    Edge(Dot { row: 0, col: 0 }, Dot { row: 0, col: 1 }),
                ),
            ])
            .expect("Draw failed");

        assert_that!(board.winner()).is_equal_to(WinnerResult::Winner(PlayerId::Two, 3));
    }

    #[test]
    fn when_size_two_is_a_tie() {
        let mut board = Board::new(2);
        // Example generated by printing out the edges played in a game:
        board
            .draw_many(vec![
                (
                    PlayerId::One,
                    Edge(Dot { row: 0, col: 0 }, Dot { row: 0, col: 1 }),
                ),
                (
                    PlayerId::Two,
                    Edge(Dot { row: 0, col: 0 }, Dot { row: 1, col: 0 }),
                ),
                (
                    PlayerId::One,
                    Edge(Dot { row: 0, col: 1 }, Dot { row: 0, col: 2 }),
                ),
                (
                    PlayerId::Two,
                    Edge(Dot { row: 0, col: 1 }, Dot { row: 1, col: 1 }),
                ),
                (
                    PlayerId::One,
                    Edge(Dot { row: 0, col: 2 }, Dot { row: 1, col: 2 }),
                ),
                (
                    PlayerId::Two,
                    Edge(Dot { row: 1, col: 0 }, Dot { row: 1, col: 1 }),
                ),
                (
                    PlayerId::One,
                    Edge(Dot { row: 1, col: 1 }, Dot { row: 1, col: 2 }),
                ),
                (
                    PlayerId::Two,
                    Edge(Dot { row: 1, col: 0 }, Dot { row: 2, col: 0 }),
                ),
                (
                    PlayerId::One,
                    Edge(Dot { row: 1, col: 2 }, Dot { row: 2, col: 2 }),
                ),
                (
                    PlayerId::Two,
                    Edge(Dot { row: 1, col: 1 }, Dot { row: 2, col: 1 }),
                ),
                (
                    PlayerId::One,
                    Edge(Dot { row: 2, col: 0 }, Dot { row: 2, col: 1 }),
                ),
                (
                    PlayerId::Two,
                    Edge(Dot { row: 2, col: 1 }, Dot { row: 2, col: 2 }),
                ),
            ])
            .expect("Draw failed");

        assert_that!(board.winner())
            .is_equal_to(WinnerResult::Tie(vec![PlayerId::One, PlayerId::Two], 2));
    }
}

mod test_box_owner {
    use super::*;

    #[test]
    fn when_players_perfectly_alternate() {
        let mut board = Board::new(2);
        board
            .draw_many(vec![
                (PlayerId::One, edge((1, 1), (1, 2))),
                (PlayerId::Two, edge((1, 1), (2, 1))),
                (PlayerId::One, edge((2, 2), (1, 2))),
                (PlayerId::Two, edge((2, 2), (2, 1))),
            ])
            .expect("Draw failed");

        assert_that!(board.box_owner(dot(1, 1)).unwrap()).is_equal_to(PlayerId::Two);
    }

    #[test]
    fn when_both_players_diverge() {
        let mut board = Board::new(10);
        board
            .draw_many(vec![
                // Start working on this box together...
                (PlayerId::One, edge((1, 1), (1, 2))),
                (PlayerId::Two, edge((1, 1), (2, 1))),
                // Go off and do some other stuff...
                (PlayerId::One, edge((7, 7), (7, 8))),
                (PlayerId::Two, edge((9, 10), (10, 10))),
                // Come back and finish that box...
                (PlayerId::One, edge((2, 2), (1, 2))),
                (PlayerId::Two, edge((2, 2), (2, 1))),
            ])
            .expect("Draw failed");

        assert_that!(board.box_owner(dot(1, 1)).unwrap()).is_equal_to(PlayerId::Two);
    }

    #[test]
    fn when_one_player_diverges() {
        let mut board = Board::new(10);
        board
            .draw_many(vec![
                // Start working on this box together...
                (PlayerId::One, edge((1, 1), (1, 2))),
                (PlayerId::Two, edge((1, 1), (2, 1))),
                (PlayerId::One, edge((2, 2), (1, 2))),
                // Player two overlooks the last edge...
                (PlayerId::Two, edge((7, 7), (7, 8))),
                // Player one claims the box
                (PlayerId::One, edge((2, 2), (2, 1))),
            ])
            .expect("Draw failed");

        assert_that!(board.box_owner(dot(1, 1)).unwrap()).is_equal_to(PlayerId::One);
    }
}

mod test_dot_chars {
    use super::*;

    #[test]
    fn at_two_way_intersections() {
        let mut board = Board::new(2);
        board
            .draw_many(vec![
                (PlayerId::One, edge((1, 1), (1, 2))),
                (PlayerId::Two, edge((1, 1), (2, 1))),
                (PlayerId::One, edge((2, 2), (1, 2))),
                (PlayerId::Two, edge((2, 2), (2, 1))),
            ])
            .expect("Draw failed");

        assert_eq!('┌', board.choose_char(dot(1, 1)).value);
        assert_eq!('└', board.choose_char(dot(2, 1)).value);
        assert_eq!('┐', board.choose_char(dot(1, 2)).value);
        assert_eq!('┘', board.choose_char(dot(2, 2)).value);
    }

    #[test]
    fn at_three_way_intersections() {
        let mut board = Board::new(2);
        board
            .draw_many(vec![
                (PlayerId::One, edge((1, 1), (0, 1))),
                (PlayerId::Two, edge((1, 1), (1, 2))),
                (PlayerId::One, edge((1, 1), (2, 1))),
            ])
            .expect("Draw failed");
        assert_eq!('├', board.choose_char(dot(1, 1)).value);

        board = Board::new(2);
        board
            .draw_many(vec![
                (PlayerId::Two, edge((1, 1), (1, 2))),
                (PlayerId::One, edge((1, 1), (2, 1))),
                (PlayerId::Two, edge((1, 1), (1, 0))),
            ])
            .expect("Draw failed");
        assert_eq!('┬', board.choose_char(dot(1, 1)).value);

        board = Board::new(2);
        board
            .draw_many(vec![
                (PlayerId::One, edge((1, 1), (0, 1))),
                (PlayerId::Two, edge((1, 1), (2, 1))),
                (PlayerId::One, edge((1, 1), (1, 0))),
            ])
            .expect("Draw failed");
        assert_eq!('┤', board.choose_char(dot(1, 1)).value);

        board = Board::new(2);
        board
            .draw_many(vec![
                (PlayerId::Two, edge((1, 1), (0, 1))),
                (PlayerId::One, edge((1, 1), (1, 2))),
                (PlayerId::Two, edge((1, 1), (1, 0))),
            ])
            .expect("Draw failed");
        assert_eq!('┴', board.choose_char(dot(1, 1)).value);
    }
}

mod test_to_string {
    use super::*;

    #[test]
    fn empty_board() {
        let board = Board::new(12);

        let expected = vec![
            "   0  1  2  3  4  5  6  7  8  9  10 11 12 ",
            " 0 ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ",
            "                                          ",
            " 1 ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ",
            "                                          ",
            " 2 ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ",
            "                                          ",
            " 3 ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ",
            "                                          ",
            " 4 ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ",
            "                                          ",
            " 5 ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ",
            "                                          ",
            " 6 ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ",
            "                                          ",
            " 7 ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ",
            "                                          ",
            " 8 ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ",
            "                                          ",
            " 9 ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ",
            "                                          ",
            " 10·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ",
            "                                          ",
            " 11·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ",
            "                                          ",
            " 12·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ·  ",
        ]
        .join("\n");

        let result = board.to_string();

        assert_eq!(
            expected,
            result,
            "\n{}",
            vec!["Expected:", &expected, "Received:", &result].join("\n")
        );
    }

    #[test]
    fn with_filled_square() {
        let mut board = Board::new(2);
        board
            .draw_many(vec![
                (PlayerId::One, edge((1, 1), (1, 2))),
                (PlayerId::Two, edge((1, 1), (2, 1))),
                (PlayerId::One, edge((1, 2), (2, 2))),
                (PlayerId::Two, edge((2, 1), (2, 2))),
            ])
            .expect("Draw failed");

        let expected = vec![
            "   0  1  2  ",
            " 0 ·  ·  ·  ",
            "            ",
            " 1 ·  ┌──┐  ",
            "      │2 │  ",
            " 2 ·  └──┘  ",
        ]
        .join("\n");

        let result = board.to_string();

        assert_eq!(
            expected,
            result,
            "\n{}",
            vec!["Expected:", &expected, "Received:", &result].join("\n")
        );
    }
}
