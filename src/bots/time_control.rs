use crate::types::piece::Color;

pub struct TimeControl {
    wtime: u32,
    btime: u32,
    winc: u32,
    binc: u32,
    move_time: Option<u32>,
}

impl TimeControl {
    pub fn new(wtime: u32, btime: u32, winc: u32, binc: u32, move_time: Option<u32>) -> Self {
        Self {
            wtime,
            btime,
            winc,
            binc,
            move_time,
        }
    }

    pub fn get_move_time(&self, turn: &usize) -> u32 {
        if let Some(move_time) = self.move_time {
            return move_time;
        }
        if turn == &Color::WHITE {
            return self.wtime / 40 + self.winc / 2;
        }
        self.btime / 40 + self.binc / 2
    }

    pub fn max() -> Self {
        Self {
            wtime: u32::MAX,
            btime: u32::MAX,
            winc: u32::MAX,
            binc: u32::MAX,
            move_time: Some(u32::MAX),
        }
    }

    #[cfg(test)]
    pub fn set_move_time(&mut self, move_time: u32) {
        self.move_time = Some(move_time);
    }
}
