use bevy::prelude::*;

use crate::chess_logic::Piece;

use super::{ChessMove, ChessState, Position};

impl ChessState {
    /// Change state
    pub fn excecute_move(&mut self, piece: Piece, pos: Position) {
        // update threatenned squares
        self.update_threat(piece, pos);

        // if this is a capture
        if self.occupied(pos) && self.capturable(pos) {
            // remove the threatened piece's targetted squares
            self.rem_threat_piece(Piece::new(self.at(pos), pos));
            // remove the targetted piece from the vector
            let remove = Piece::new(self.board[pos.int()], pos);
            let pieces = &mut self.pieces[!self.turn as usize];
            // if you panic here something went wrong with syncing board and piece vecs
            pieces.swap_remove(pieces.iter().position(|&p| p == remove).unwrap());
        }
        // move the thing there
        self.board[pos.int()] = self.board[piece.position.int()];
        self.board[piece.position.int()] = default();

        // update the pieces
        self.pieces[self.turn as usize].iter_mut().find(|&&mut p| p == piece).unwrap().position =
            pos;
        self.turn = !self.turn;
    }
}
