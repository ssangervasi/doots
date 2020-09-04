use spectral::assert_that;

use doots::ai::doot;
use doots::game::board::{edge, Board};
use doots::players::player::{Player, PlayerId};

#[test]
fn test_draw_on_open_board() {
    let ai = doot::AI::new(PlayerId::One);
    let board = Board::new(2);
    let play = ai.play(board);

    assert_that!(play).is_equal_to(edge((2, 1), (2, 2)));
}

#[test]
fn test_draw_to_complete_box() {
    let ai = doot::AI::new(PlayerId::Two);
    let mut board = Board::new(2);
    board
        .draw_many(vec![
            (PlayerId::One, edge((0, 0), (0, 1))),
            (PlayerId::Two, edge((0, 0), (1, 0))),
            (PlayerId::One, edge((1, 0), (1, 1))),
        ])
        .expect("Draw failed");

    let play = ai.play(board);
    assert_that!(play).is_equal_to(edge((0, 1), (1, 1)));
}
