use crate::bitboard::BitBoard;


struct Board {
    pawns: BitBoard,
    rooks: BitBoard,
    knights: BitBoard,
    bishops: BitBoard,
    queens: BitBoard,
    kings: BitBoard,
    white_pieces: BitBoard,
    black_pieces: BitBoard,
}
