use crate::board::Board;
use crate::bots::bot::Bot;
use crate::bots::time_control::TimeControl;
use crate::moves::moves_utils::Move;
use crate::{moves::move_mask_gen::MoveGenMasks, utils::zobrist::ZobristHasher};
use std::sync::atomic::AtomicBool;

use std::io::stdin;
use std::process;
use std::sync::atomic::Ordering;
use std::sync::mpsc::{self, Receiver};
use std::thread;

pub static UCI_STOP: AtomicBool = AtomicBool::new(false);

pub struct UCIGame {
    move_gen_masks: MoveGenMasks,
    hasher: ZobristHasher,
    bot: Bot,
    board: Board,
}

impl UCIGame {
    pub fn new() -> Self {
        let hasher = ZobristHasher::load();
        Self {
            move_gen_masks: MoveGenMasks::load(),
            hasher: ZobristHasher::load(),
            bot: Bot::default(),
            board: Board::new(&hasher),
        }
    }

    pub fn uci_io_loop(&mut self) {
        let (tx, rx) = mpsc::channel();

        let io_thread = thread::spawn(move || loop {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            input = input.trim().to_owned();
            if input.as_str() == "stop" {
                UCI_STOP.store(true, Ordering::Relaxed);
            } else if input.as_str() == "quit" {
                process::exit(0);
            }

            tx.send(input).unwrap();
        });

        self.receive_command(rx);

        io_thread.join().unwrap();
    }

    fn receive_command(&mut self, rx: Receiver<String>) {
        loop {
            if let Ok(input) = rx.try_recv() {
                if !input.is_empty() {
                    self.handle_command(input);
                }
            }
        }
    }

    fn handle_command(&mut self, input: String) {
        let mut input: Vec<&str> = input.split_whitespace().collect();
        let command = input.remove(0);

        match command {
            "debug" => (),
            "go" => self.uci_go(input),
            "is_ready" => self.uci_is_ready(),
            "ponderhit" => (), // later (or never)
            "position" => self.uci_position(input),
            "setoption" => (), // later
            "stop" => (),      // done in uci_io_loop
            "uci" => self.uci_uci(),
            "ucinewgame" => (),                            // not necessary?
            "quit" => (),                                  // done in uci_io_loop
            "print_board" => println!("\n{}", self.board), // not UCI command
            _ => (),
        }
    }

    fn uci_go(&mut self, args: Vec<&str>) {
        UCI_STOP.store(false, Ordering::Relaxed);
        let mut wtime = u32::MAX;
        let mut btime = u32::MAX;
        let mut winc = 0;
        let mut binc = 0;
        let mut move_time: Option<u32> = None;

        for i in 0..args.len() {
            match args[i] {
                "infinite" => self.bot.set_depth(u8::MAX),
                "searchmoves" => (),
                "ponder" => (),
                "wtime" => wtime = args[i + 1].parse::<u32>().unwrap(),
                "btime" => btime = args[i + 1].parse::<u32>().unwrap(),
                "winc" => winc = args[i + 1].parse::<u32>().unwrap(),
                "binc" => binc = args[i + 1].parse::<u32>().unwrap(),
                "movestogo" => (),
                "depth" => self.bot.set_depth(args[i + 1].parse::<u8>().unwrap()),
                "nodes" => (),
                "mate" => (),
                "movetime" => move_time = Some(args[i + 1].parse::<u32>().unwrap()),
                _ => continue,
            }
        }

        let time_control = TimeControl::new(wtime, btime, winc, binc, move_time);

        self.bot.set_time_control(time_control);

        let bot_move = self
            .bot
            .get_best_move(&mut self.board, &self.move_gen_masks, &self.hasher);
        self.board.make_move(&bot_move, &self.hasher);
        println!("bestmove {}", bot_move.to_long_string());
    }

    fn uci_uci(&self) {
        println!("id name {}", env!("CARGO_PKG_NAME"));
        println!("id author {}", env!("CARGO_PKG_AUTHORS"));
        // TODO: add options
        println!("uciok");
    }

    fn uci_position(&mut self, mut args: Vec<&str>) {
        let start_pos = args.remove(0);

        if start_pos == "startpos" {
            self.board = Board::new(&self.hasher)
        } else if start_pos == "fen" {
            let mut fen_string = args.remove(0).to_owned();
            for _ in 0..5 {
                fen_string.push(' ');
                fen_string.push_str(args.remove(0));
            }
            self.board = match Board::from_fen(&fen_string, &self.hasher) {
                Ok(board) => board,
                Err(e) => panic!("{}", e),
            };
        } else {
            panic!("unexpected position argument")
        }

        if !args.is_empty() && args.remove(0) == "moves" {
            for move_str in args {
                let move_to_make = Move::from_long_str(move_str);
                self.board
                    .check_and_make_move(&move_to_make, &self.move_gen_masks, &self.hasher)
            }
        }
    }

    fn uci_is_ready(&self) {
        println!("readyok");
    }
}

impl Default for UCIGame {
    fn default() -> Self {
        Self::new()
    }
}
