mod player;
use player::Player;

mod cell;
use cell::Cell;

use primes::{Sieve, PrimeSet};
use std::io::{self, Write};

pub struct TicTacToe {
    board: [[Cell; 3]; 3],
    current_player: Player,
    player_one_score: u64,
    player_two_score: u64,

    win_ways: [u64; 8],
}

impl Default for TicTacToe {
    fn default() -> Self {
        // getting solutions

        Self {
            current_player: Player::None,
            board: TicTacToe::initizalize_boardgame(),
            player_one_score: 1,
            player_two_score: 1,
            win_ways: TicTacToe::get_possible_solutions(),
        }
    }
}

impl TicTacToe {
    fn initizalize_boardgame() -> [[Cell; 3]; 3] {
        let mut pset = Sieve::new();
        let cell_values = pset.iter().take(9);
        let mut tmp_board = [[Cell::default(); 3]; 3];
        // initializing cells
        for (i, val) in cell_values.enumerate() {
            tmp_board[i/3][i%3].value = val;
        }

        tmp_board
    }

    fn get_possible_solutions() -> [u64; 8] {
        /* Return all solutions
         * It's working but not good enouth, this is a scratch for a 
         * code I'll need when I implement the NxN TicTacToe 
         */
        let mut primes = Sieve::new();
        let mut solutions: [u64; 8] = [0; 8]; 
        // getting lines solutions
        for li in 0..3 { 
            let new_solution = primes.iter().skip(li*3).step_by(1).take(3)
                .fold(1, |acc, x| acc * x);
            
            solutions[li] = new_solution;
        }
        // getting cols solutions
        for ci in 0..3 { 
            let new_solution = primes.iter().skip(ci).step_by(3).take(3)
                .fold(1, |acc, x| acc * x);
            
            solutions[3 + ci] = new_solution;
        }
        // getting the bisectors solutions
        for bi in 0..2 {
            let new_solution = primes.iter().skip(2*bi).step_by(4 - 2*bi).take(3)
                .fold(1, |acc, x| acc * x);
            
            solutions[6 + bi] = new_solution;
        }
        solutions
    }

    fn draw_board(&self) {
        let board_lines = self.board.map(|line| {
                                    line.map(|cell| {
                                        cell.draw()
                                    }).join(" | ")
                                });
        println!("{}", board_lines.join("\n"));
    }


    fn switch_player(&mut self) {
        match self.current_player {
            Player::None => self.current_player = Player::One,
            Player::One => self.current_player = Player::Two,
            Player::Two => self.current_player = Player::One,
        }
    }

    fn get_winner(&self) -> Player {
       for solution in self.win_ways {
           if self.player_one_score % solution == 0 { return Player::One }
           if self.player_two_score % solution == 0 { return Player::Two }
       }
       Player::None
    }

    fn any_empty_cell(&self) -> bool {
        for line in self.board {
            for cell in line {
                if cell.is_empty() { return true; }
            }
        }
        false
    }

    fn finished(&self) -> bool {
        !Player::is_none(self.get_winner()) | !self.any_empty_cell()
    }

    fn get_safe_position(param_name: &str) -> usize {
        let param : usize;

        loop {
            let mut input = String::new();

            print!("{} of cell: ",param_name);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            match input.trim().parse::<usize>() {
                Ok(num) => {
                    if num < 1 || num > 3 {
                        println!("{} must be a number between 1, 2 and 3", param_name);
                        continue;
                    }
                    param = num - 1;
                    break
                }
                Err(_) => {
                    println!("{} must be a number between 1, 2 and 3", param_name);
                    continue
                }
            }
        }
        param
    }

    fn select_cell(&mut self, player: Player, line: usize, col: usize) -> bool {
        let cell = self.board[line][col];

        match cell {
            _ if cell.is_empty() => {
                match player {
                    Player::One => {
                        self.player_one_score *= cell.value;
                        self.board[line][col].set_to_x();
                    },
                    Player::Two => {
                        self.player_two_score *= cell.value;
                        self.board[line][col].set_to_circle();
                    },
                    Player::None => { /* Nothing */ },
                }
                true
            },
            _ => false,
        }
    }

    fn new_round(&mut self, player: Player) {
        match player {
            Player::One => println!("Player One"),
            Player::Two => println!("Player Two"),
            _ => println!(""),
        }
        println!("What's the cell that you want?");
        loop {
            // get line input
            let line = TicTacToe::get_safe_position("line");
            let col = TicTacToe::get_safe_position("column");
            // trying to select cell
            if self.select_cell(player, line, col) {
                break;
            }
        }
    }

    fn display(&mut self) {
        self.switch_player();
        println!("{}", "=".repeat(30));
        println!("   Player {:?}", self.current_player);

        println!("{}", "-".repeat(20));

        self.draw_board();

        println!("{}", "-".repeat(20));
    }

    fn congrats_winner(&self) {
        let winner = self.get_winner();

        match winner {
            Player::One => println!("Player One, you win!"),
            Player::Two => println!("Player Two, you win!"),
            _ => println!("It was a tie"),
        }
        println!("{}", "-".repeat(20));
    }

    pub fn start_game(&mut self) {
        loop {
            self.display();
            self.new_round(self.current_player);

            if self.finished() { break }
        }
        self.display();
        self.congrats_winner();
    }
}


fn main() {
    let mut tic_tac_toe = TicTacToe::default();

    tic_tac_toe.start_game();
}
