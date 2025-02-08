use crate::board::Board;
use crate::bots::bot::Bot;
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
        Self {
            move_gen_masks: MoveGenMasks::load(),
            hasher: ZobristHasher::load(),
            bot: Bot::default(),
            board: Board::default(),
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

        for i in 0..args.len() {
            match args[i] {
                "infinite" => self.bot.set_depth(u8::MAX),
                "searchmoves" => (),
                "ponder" => (),
                "wtime" => (),
                "btime" => (),
                "winc" => (),
                "binc" => (),
                "movestogo" => (),
                "depth" => self.bot.set_depth(args[i + 1].parse::<u8>().unwrap()),
                "nodes" => (),
                "mate" => (),
                "movetime" => (),
                _ => continue,
            }
        }

        let (bot_move, new_board) =
            self.bot
                .get_best_move(&self.board, &self.move_gen_masks, &self.hasher);
        self.board = new_board;
        println!("bestmove {}", bot_move.to_long_string());
        println!("{}", self.board);
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
            self.board = Board::default()
        } else if start_pos == "fen" {
            let mut fen_string = args.remove(0).to_owned();
            for _ in 0..5 {
                fen_string.push(' ');
                fen_string.push_str(args.remove(0));
            }
            self.board = match Board::from_fen(&fen_string) {
                Ok(board) => board,
                Err(e) => panic!("{}", e),
            };
        } else {
            panic!("unexpected position argument")
        }

        if !args.is_empty() && args.remove(0) == "moves" {
            for move_str in args {
                let move_to_make = Move::from_long_str(move_str);
                match self.board.check_and_make_move(
                    &move_to_make,
                    &self.move_gen_masks,
                    &self.hasher,
                ) {
                    Some(new_board) => self.board = new_board,
                    None => panic!("Could not make move {}", move_str),
                }
            }
        }
    }

    fn uci_is_ready(&self) {
        println!("readyok");
    }
}
