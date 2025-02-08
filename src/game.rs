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
use std::time::Duration;

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
            bot: Bot::new(5),
            board: Board::default(),
        }
    }

    pub fn uci_io_loop(&mut self) {
        let (tx, rx) = mpsc::channel();

        let io_thread = thread::spawn(move || loop {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            tx.send(input.trim().to_owned()).unwrap();
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
            "stop" => UCI_STOP.store(true, Ordering::Relaxed),
            "uci" => self.uci_uci(),
            "ucinewgame" => (), // not necessary?
            "quit" => self.uci_quit(),
            "print_board" => println!("\n{}", self.board), // not UCI command
            _ => (),
        }
    }

    fn uci_go(&self, args: Vec<&str>) {
        UCI_STOP.store(false, Ordering::Relaxed);
    }

    fn uci_uci(&self) {
        println!("id name {}", env!("CARGO_PKG_NAME"));
        println!("id author {}", env!("CARGO_PKG_AUTHORS"));
        // TODO: add options
        println!("uciok");
    }

    fn uci_quit(&self) {
        UCI_STOP.store(true, Ordering::Relaxed);
        thread::sleep(Duration::from_millis(500));
        process::exit(0);
    }

    fn uci_position(&mut self, mut args: Vec<&str>) {
        let start_pos = args.remove(0);

        if start_pos == "startpos" {
            self.board = Board::default()
        } else if start_pos == "fen" {
            let fen = args.remove(0);
            self.board = match Board::from_fen(fen) {
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
