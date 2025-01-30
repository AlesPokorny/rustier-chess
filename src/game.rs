use crate::board::Board;
use crate::moves::move_mask_gen::MoveGenMasks;
use crate::moves::moves_calculation::{get_all_moves, is_square_in_check};
use crate::moves::moves_utils::Move;
use crate::piece::Pieces;
use crate::square::Square;

pub struct Game {
    pub board: Board,
    pub move_gen_masks: MoveGenMasks,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: Board::default(),
            move_gen_masks: MoveGenMasks::load(),
        }
    }
}

impl Game {
    pub fn get_legal_moves(&self) -> Vec<(Move, Board)> {
        let all_moves: Vec<(Move, Board)> = get_all_moves(&self.board, &self.move_gen_masks)
            .into_iter()
            .filter_map(|the_move| {
                let new_board = self.try_move(&the_move);
                if is_square_in_check(
                    &new_board.pieces[new_board.state.opponent][Pieces::KING].get_one(),
                    &new_board,
                    &self.move_gen_masks,
                ) {
                    return Some((the_move, new_board));
                }
                None
            })
            .collect();

        all_moves
    }

    pub fn try_move(&self, the_move: &Move) -> Board {
        let origin = the_move.get_origin();
        let destination = the_move.get_destination();
        let is_capture = self.board.colors[self.board.state.opponent].read_square(&destination);

        let mut new_board = self.board;
        new_board.state.increment_half_move();

        new_board.colors[new_board.state.turn].set_zero(&origin);
        new_board.colors[new_board.state.turn].set_one(&destination);
        for (piece_type, piece_bitboard) in new_board.pieces[new_board.state.turn]
            .iter_mut()
            .enumerate()
        {
            if piece_bitboard.read_square(&origin) {
                piece_bitboard.set_zero(&origin);
                piece_bitboard.set_one(&destination);
                if piece_type == Pieces::PAWN {
                    new_board.state.reset_half_move();
                }
                break;
            }
        }

        if is_capture {
            new_board.colors[new_board.state.opponent].set_zero(&destination);
            for piece_bitboard in new_board.pieces[new_board.state.opponent].iter_mut() {
                if piece_bitboard.read_square(&origin) {
                    piece_bitboard.set_zero(&origin);
                    break;
                }
            }
            new_board.state.reset_half_move();
        }

        match the_move.special_move() {
            0 => (),
            1 => {
                // 1 promotion
                new_board.clear_piece(&origin, Pieces::PAWN, new_board.state.turn);
                new_board.pieces[new_board.state.turn][the_move.get_promotion_piece()]
                    .set_one(&destination);
                new_board.colors[new_board.state.turn].set_one(&destination);
            }
            2 => {
                // 2 en passant
                let square_to_clear = Square::new(origin.get_rank() * 8 + destination.get_file());
                new_board.clear_piece(&square_to_clear, Pieces::PAWN, new_board.state.opponent);
            }
            3 => {
                // 3 castling
                let rook_origin_file: u8;
                let rook_destination_file: u8;
                if destination.get_file() == 2 {
                    // long
                    rook_origin_file = 0;
                    rook_destination_file = 3;
                } else {
                    // short
                    rook_origin_file = 7;
                    rook_destination_file = 5;
                }
                let rank = origin.get_rank() * 8;
                let rook_origin = Square::new(rank + rook_origin_file);
                let rook_destination = Square::new(rank + rook_destination_file);
                new_board.move_piece(
                    &rook_origin,
                    &rook_destination,
                    Pieces::ROOK,
                    new_board.state.turn,
                );
            }
            _ => panic!("Boom, invalid special move"),
        }

        new_board.state.increment_full_move();
        new_board.state.change_turn();

        new_board
    }
}
