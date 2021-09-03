#![allow(
	unused_must_use, 
	unused_assignments, 
	dead_code, 
	unused_variables
)]

use std::fmt;

type Board = Vec<Vec<Option<Player>>>;

/// Simple enum, so deriving Clone, Copy will not incur a performance cost
#[derive(Debug, Clone, Copy, PartialEq)]
enum Player {
	Red, // matches to char 'o'
	Black, // matches to char 'x'
}

impl fmt::Display for Player {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Player::Red => write!(f, "o"),
			Player::Black => write!(f, "x"),
		}
	}
}

pub struct ConnectFour {
	board: Board, 
	turn: Player,  
}

impl ConnectFour {
	pub fn new() -> Self {
		ConnectFour {
			board: vec![vec![None; 7]; 6],
			turn: Player::Red,
		}
	}

	pub fn take_turn(&mut self, col: usize) -> Result<bool, &str> {
		let mut flag = false;
		for i in (0..self.board.len()).rev() {
			if self.board[i][col].is_some() {
				continue;
			} else {
				self.board[i][col] = Some(self.turn);
				match &self.turn {
					Player::Red => self.turn = Player::Black,
					Player::Black => self.turn = Player::Red,
				}
				flag = true;
				break;
			}
		}

		if !flag {
			Err("Invalid move, try again... ")
		} else {
			Ok(self.check_win())
		}
		
	}

	fn check_win(&self) -> bool {
		for row in (0..self.board.len()).rev() {
			for col in (0..self.board[row].len()).rev() {
				// if no piece found continue
				if self.board[row][col].is_none() {
					continue;
				}

				// piece that we are checking
				// unwrap will not panic because already checked that is not none 
				let piece = self.board[row][col].unwrap();

				// traverse diagonal, col, and row to find if win condition met
				// diagonal --> (row + 1, col + 1), (row + 1, col - 1), (row - 1, col + 1), (row - 1, col - 1)
				// two directions:
				// 	pos diagonal = [(row + 1, col - 1), (row - 1, col + 1)]
				//    neg diagonal = [(row + 1, col + 1), (row -1, col - 1)]
				// col --> [(row + 1, col), (row - 1, col)]
				// row --> [(row, col + 1), (row, col - 1)]
				// while direction valid traverse until win condition met or direction no longer valid
				// Direction valid --> 
				// 	if in bounds of board
				//		if next piece matches prev	
				// lastly check if draw by checking if no None types are present

				
				// TODO: Implement each directional check (consider placing in seperate util file)
				if self.check_col(piece, row, col) { return true; }
				if self.check_row(piece, row, col) { return true }
				// if self_check_pos_diag(piece, row, col) {return true }
				// if self_check_neg_diag(piece, row, col) {return true }

			}
		}

		// unimplemented!();
		false
	}

	fn check_col(&self, piece: Player, row: usize, col: usize) -> bool {
		// win condition is ct >= 4
		let mut ct = 1; 
		
		// check down direction
		for y in 1..= 3 {
			if row + y > self.board.len() - 1 ||
				self.board[row + y][col].is_none() || 
				self.board[row + y][col].unwrap() != piece { 
					break; 
			}
			ct += 1;
		}

		// check up direction
		for y in 1..=3 {
			if (row as i32 - y as i32) < 0 ||
				self.board[row - y][col].is_none() || 
				self.board[row - y][col].unwrap() != piece { 
					break; 
			}
			ct += 1;
		}

		// check win condition
		if ct >= 4 {
			true
		} else {
			false
		}
	}

	fn check_row(&self, piece: Player, row: usize, col: usize) -> bool {
		// win condition is ct >= 4
		let mut ct = 1; 
		
		// check right direction
		for x in 1..= 3 {
			if col + x > self.board[row].len() - 1 ||
				self.board[row][col + x].is_none() || 
				self.board[row][col + x].unwrap() != piece { 
					break; 
			}
			ct += 1;
		}

		// check left direction
		for x in 1..=3 {
			if (col as i32 - x as i32) < 0 ||
				self.board[row][col - x].is_none() || 
				self.board[row][col - x].unwrap() != piece { 
					break; 
			}
			ct += 1;
		}

		// check win condition
		if ct >= 4 {
			true
		} else {
			false
		}
	}

	fn check_pos_diag(&self, piece: Player, row: usize, col: usize) -> bool {
		unimplemented!();
	}

	fn check_neg_diag(&self, piece: Player, row: usize, col: usize) -> bool {
		unimplemented!();
	}


	fn reset(&mut self) {
		self.board = vec![vec![None; 7]; 6]; 		
		self.turn = Player::Red;
	}
}

impl fmt::Display  for ConnectFour {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut res = String::new();
		res += "  0 1 2 3 4 5 6 \n";
		// res += " _ _ _ _ _ _ _\n";
		for row in &self.board {
			res += "|";
			for val in row {
				match val {
					Some(player) => {
						res += " ";
						res += &player.to_string();
					},
					None => res += " -",
				}
			}
			res += " |\n";
		}

		res += " ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾ \n";
		// println!("{}", res);

		write!(f, "{}", res)
	}

}

#[cfg(test)]
mod test_connect_four {
	use super::*;

	#[test]
	fn test_no_win() {
		let mut game = ConnectFour::new();
		println!("{}", game);
		assert_eq!(game.take_turn(2), Ok(false)); // o
		game.take_turn(2); // x
		game.take_turn(2); // o
		game.take_turn(4); // x
		assert_eq!(game.take_turn(3), Ok(false)); // o
		game.take_turn(4); // x
		game.take_turn(2); // x
		game.take_turn(2); // o
		game.take_turn(2); // x
		assert_eq!(game.take_turn(2), Err("Invalid move, try again... ")); // o
	}

	#[test]
	fn test_col_win() {
		let mut game = ConnectFour::new();
		println!("{}", game);

		game.take_turn(2); // o
		game.take_turn(4); // x
		game.take_turn(2); // o
		game.take_turn(4); // x
		game.take_turn(2); // o
		assert_eq!(game.take_turn(4), Ok(false)); // x
		assert_eq!(game.take_turn(2), Ok(true)); // o
		println!("{}", game);

		// does not go out of bounds
		game.reset();
		game.take_turn(2); // o
		game.take_turn(4); // x
		game.take_turn(2); // o
		game.take_turn(4); // x
		game.take_turn(2); // o
		game.take_turn(2); // x
		game.take_turn(4); // o
		game.take_turn(2); // x
		game.take_turn(4); // o
		game.take_turn(2); // x
		assert_eq!(game.take_turn(4), Ok(false)); // o
		game.take_turn(2); // x -> invalid move error here
		game.take_turn(3); // x
		assert_eq!(game.take_turn(4), Ok(true)); // o -> win here
		println!("{}", game);


	}
}