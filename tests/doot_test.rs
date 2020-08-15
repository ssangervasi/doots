use doots::ai::doot::Doot;
use doots::game::board::{edge, Board};
use doots::players::player::{Player, PlayerId};

#[test]
fn test_draw_on_open_board() {
    let doot = Doot::new(PlayerId::One);
    let board = Board::new(2);
    let play = doot.play(board);

    assert_eq!(edge((0, 0), (0, 1)), play);
}
