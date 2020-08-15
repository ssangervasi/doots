use spectral::boolean::BooleanAssertions;
use spectral::{assert_that, asserting};

use doots::game::board::{dot, edge, Board};
use doots::players::player::PlayerId;

#[test]
fn it_calculates_dot_count() {
    let board = Board::new(2);
    assert_that!(board.dot_count()).is_equal_to(9);
}

#[test]
fn test_is_free() {
    let mut board = Board::new(2);
    board.draw(edge((0, 0), (0, 1))).expect("Draw failed!");

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
            edge((1, 1), (1, 2)),
            edge((1, 1), (2, 1)),
            edge((2, 2), (1, 2)),
            edge((2, 2), (2, 1)),
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

#[test]
fn test_box_owner_when_players_perfectly_alternate() {
    let mut board = Board::new(2);
    board
        .draw_many(vec![
            edge((1, 1), (1, 2)),
            edge((1, 1), (2, 1)),
            edge((2, 2), (1, 2)),
            edge((2, 2), (2, 1)),
        ])
        .expect("Draw failed");

    assert_that!(board.box_owner(dot(1, 1)).unwrap()).is_equal_to(PlayerId::Two);
}

mod test_box_owner {
    use super::*;

    #[test]
    fn test_box_owner_when_both_players_diverge() {
        let mut board = Board::new(10);
        board
            .draw_many(vec![
                // Start working on this box together...
                edge((1, 1), (1, 2)),
                edge((1, 1), (2, 1)),
                // Go off and do some other stuff...
                edge((7, 7), (7, 8)),
                edge((9, 10), (10, 10)),
                // Come back and finish that box...
                edge((2, 2), (1, 2)),
                edge((2, 2), (2, 1)),
            ])
            .expect("Draw failed");

        assert_that!(board.box_owner(dot(1, 1)).unwrap()).is_equal_to(PlayerId::Two);
    }

    #[test]
    fn test_box_owner_when_one_player_diverges() {
        let mut board = Board::new(10);
        board
            .draw_many(vec![
                // Start working on this box together...
                edge((1, 1), (1, 2)),
                edge((1, 1), (2, 1)),
                edge((2, 2), (1, 2)),
                // Player two overlooks the last edge...
                edge((7, 7), (7, 8)),
                // Player one claims the box
                edge((2, 2), (2, 1)),
            ])
            .expect("Draw failed");

        assert_that!(board.box_owner(dot(1, 1)).unwrap()).is_equal_to(PlayerId::One);
    }
}

#[test]
fn test_dot_chars_at_two_way_intersections() {
    let mut board = Board::new(2);
    board
        .draw_many(vec![
            edge((1, 1), (1, 2)),
            edge((1, 1), (2, 1)),
            edge((2, 2), (1, 2)),
            edge((2, 2), (2, 1)),
        ])
        .expect("Draw failed");

    assert_eq!('┌', board.choose_char(dot(1, 1)).value);
    assert_eq!('└', board.choose_char(dot(2, 1)).value);
    assert_eq!('┐', board.choose_char(dot(1, 2)).value);
    assert_eq!('┘', board.choose_char(dot(2, 2)).value);
}

#[test]
fn test_dot_chars_at_three_way_intersections() {
    let mut board = Board::new(2);
    board
        .draw_many(vec![
            edge((1, 1), (0, 1)),
            edge((1, 1), (1, 2)),
            edge((1, 1), (2, 1)),
        ])
        .expect("Draw failed");
    assert_eq!('├', board.choose_char(dot(1, 1)).value);

    board = Board::new(2);
    board
        .draw_many(vec![
            edge((1, 1), (1, 2)),
            edge((1, 1), (2, 1)),
            edge((1, 1), (1, 0)),
        ])
        .expect("Draw failed");
    assert_eq!('┬', board.choose_char(dot(1, 1)).value);

    board = Board::new(2);
    board
        .draw_many(vec![
            edge((1, 1), (0, 1)),
            edge((1, 1), (2, 1)),
            edge((1, 1), (1, 0)),
        ])
        .expect("Draw failed");
    assert_eq!('┤', board.choose_char(dot(1, 1)).value);

    board = Board::new(2);
    board
        .draw_many(vec![
            edge((1, 1), (0, 1)),
            edge((1, 1), (1, 2)),
            edge((1, 1), (1, 0)),
        ])
        .expect("Draw failed");
    assert_eq!('┴', board.choose_char(dot(1, 1)).value);
}

#[test]
fn test_to_string_empty_board() {
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
fn test_to_string_with_filled_square() {
    let mut board = Board::new(2);
    board
        .draw_many(vec![
            edge((1, 1), (1, 2)),
            edge((1, 1), (2, 1)),
            edge((1, 2), (2, 2)),
            edge((2, 1), (2, 2)),
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
