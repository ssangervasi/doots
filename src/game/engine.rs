use crate::game::board::{Board, BoardSize, WinnerResult};
use crate::players::choose::choose;
use crate::utils::{pad_end, pad_out};

pub struct Opts {
    pub board_size: BoardSize,
    pub player_two: String,
    pub player_one: String,
    pub quiet: bool,
}

pub fn run_game(opts: &Opts) -> Result<(), String> {
    let mut board = Board::new(opts.board_size);

    print!(
        "{}",
        vec![
            format!("· {} ·", pad_end("", "─", 40)),
            format!("│ {} │", pad_out("Doots & Booxes", " ", 40)),
            format!(
                "│ {} │",
                pad_out(
                    &format!(
                        "Playing with {} squares ({}x{} dots)",
                        board.size(),
                        board.dot_size(),
                        board.dot_size()
                    ),
                    " ",
                    40
                )
            ),
            format!("· {} ·", pad_end("", "─", 40)),
        ]
        .join("\n")
    );

    // I'm being pretty zealos about not using the player struct's Id in order
    // to prevent a player implementation from lying about where it actually
    // falls in the turn order.
    let players = choose(&opts.player_one, &opts.player_two);
    let mut player_index = 0;
    let mut streak_count = 0;

    for turn in 0..(board.edge_count() as usize) {
        let (player_id, player) = &players[player_index];

        if !opts.quiet {
            print!("\n\n{}\n\n", board.to_string());

            if streak_count == 0 {
                println!("Turn #{}: Player {}", turn + 1, player_id);
            } else {
                println!("Streak {}! Player {}", streak_count, player_id);
            }
        }

        let owned_count_before_play = board.owned_boxes_count(*player_id);

        // Note that the board clone is intentional as we don't want our
        // players to have any way of mutating the offical board state.
        let player_edge = player.play(board.clone());
        match board.draw((*player_id, player_edge)) {
            Err(_) => {
                return Err(format!(
                    "Player {} ({}) attempted to draw an invalid edge: {}",
                    player_id,
                    player.name(),
                    player_edge,
                ));
            }
            _ => {}
        };
        if !opts.quiet {
            println!("Player {} drew: {}", player_id, player_edge);
        }
        let owned_count_after_play = board.owned_boxes_count(*player_id);

        if owned_count_before_play < owned_count_after_play {
            if !opts.quiet {
                println!("Player {} finished a box!", player_id);
            }
            streak_count += 1;
        } else {
            player_index = (player_index + 1) % players.len();
            streak_count = 0;
        }
    }

    print!("\n\n{}\n\n", board.to_string());

    let winner_message = match board.winner() {
        WinnerResult::Winner(winner_id, winner_count) => {
            let (_, winner) = players.iter().find(|(id, _)| *id == winner_id).unwrap();
            format!(
                "Player {} ({}) wins with {} boxes!",
                winner_id,
                winner.name(),
                winner_count
            )
        }
        WinnerResult::Tie(tied_ids, tied_count) => format!(
            "A tie between {:?} with {} boxes each.",
            tied_ids, tied_count
        ),
        WinnerResult::None => "I think something went wrong...".to_string(),
    };

    print!(
        "{}",
        vec![
            format!("· {} ·", pad_end("", "─", winner_message.len())),
            format!("│ {} │", pad_out("GAME OVER", " ", winner_message.len())),
            format!("│ {} │", winner_message),
            format!("· {} ·", pad_end("", "─", winner_message.len())),
        ]
        .join("\n")
    );

    Ok(())
}
