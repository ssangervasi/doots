use doots::board::{dot, edge, Board};

#[test]
fn it_calculates_dot_count() {
    let board = Board::new(2);
    assert_eq!(9, board.dot_count())
}

#[test]
fn test_is_free() {
    let mut board = Board::new(2);
    board.draw(edge((0, 0), (0, 1))).expect("Draw failed!");

    assert_eq!(true, board.is_free(edge((0, 0), (1, 0))));
    assert_eq!(false, board.is_free(edge((0, 0), (0, 1))));
    assert_eq!(false, board.is_free(edge((0, 1), (0, 0))));
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
fn test_to_string() {
    let mut board = Board::new(2);
    board
        .draw_many(vec![
            edge((1, 1), (1, 2)),
            edge((1, 1), (2, 1)),
            edge((1, 2), (2, 2)),
            edge((2, 1), (2, 2)),
        ])
        .expect("Draw failed");

    #[rustfmt::skip]
    let expected = vec![
  	  "   0  1  2 ",
	  " 0 ·  ·  ·",
	  " 1 ·  ┌──┐",
	  " 2 ·  └──┘",
	].join("\n");

    let result = board.to_string();

    assert_eq!(
        expected,
        result,
        "\n{}",
        vec!["Expected:", &expected, "Received:", &result].join("\n")
    );
}
