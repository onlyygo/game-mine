use std::io;
use rand::Rng;

const MAX_CHESS: usize = 36;
const MAX_MINE: usize = 10;
const MINE_ID: i32 = -1;
fn t(value: i32) -> String {
	if value==0 {
		return String::from("   ");
	}else if value==-1 {
		return String::from(" @ ");
	}else {
		return String::from(format!(" {} ", value));
	}
}

fn n2c(i: usize) -> u8{
	if i<=9 {
		return i as u8 + b'0';
	}else {
		return (i-10) as u8 + b'a';
	}
}
fn show_chess(chess: & [[i32; MAX_CHESS]; MAX_CHESS], mask: & [[i32; MAX_CHESS]; MAX_CHESS]) {
    for row in 0..MAX_CHESS {
    	if row == 0 {
    		print!("   ");
    		for col in 0..MAX_CHESS {
	        	print!(" {} ", n2c(col) as char);
	        }
	        println!("");
    	}
        for col in 0..MAX_CHESS {
        	if col == 0 {
        		print!(" {} ", n2c(row) as char);
        	}
        	if mask[row][col] == 0 {
        		print!(" * ");
        	}else {
        		print!("{}", t(chess[row][col]));
        	}
        }
        println!("");
    }
    println!("############");
}

fn add_flag(chess: &mut [[i32; MAX_CHESS]; MAX_CHESS]) {
    for row in 0..MAX_CHESS {
        for col in 0..MAX_CHESS {
            if chess[row][col]==MINE_ID {
            	add_flag_kernel(chess, row, col);
            }
        }
    }
}

fn max(a: i32, b: i32) -> i32{
	if a>b {
		return a;
	}
	return b;
}

fn min(a: i32, b: i32) -> i32{
	if a<b {
		return a;
	}
	return b;
}

fn add_flag_kernel(chess: &mut [[i32; MAX_CHESS]; MAX_CHESS], row: usize, col: usize){
	let start_row: usize = max(0,                   row as i32 -1) as usize;
	let end_row: usize   = min(MAX_CHESS as i32 -1, row as i32 +1) as usize;
	let start_col: usize = max(0,                   col as i32 -1) as usize;
	let end_col: usize   = min(MAX_CHESS as i32 -1, col as i32 +1) as usize;
	for row2 in start_row..(end_row+1) {
		for col2 in start_col..(end_col+1) {
			if chess[row2][col2]>=0 {
				chess[row2][col2] += 1;
			}
		}
	}
}

fn spread(chess: &mut [[i32; MAX_CHESS]; MAX_CHESS], mask: &mut [[i32; MAX_CHESS]; MAX_CHESS], row: usize, col: usize){
	let start_row: usize = max(0,                   row as i32 -1) as usize;
	let end_row: usize   = min(MAX_CHESS as i32 -1, row as i32 +1) as usize;
	let start_col: usize = max(0,                   col as i32 -1) as usize;
	let end_col: usize   = min(MAX_CHESS as i32 -1, col as i32 +1) as usize;
	for row2 in start_row..(end_row+1) {
		for col2 in start_col..(end_col+1) {
			if row2 == row && col2 == col {
				continue;
			}
			if mask[row2][col2] == 1 {
				continue;
			}
			mask[row2][col2] = 1;
			if chess[row2][col2] == 0 {
				spread(chess, mask, row2, col2);
			}
		}
	}
}

fn is_victory(chess: & [[i32; MAX_CHESS]; MAX_CHESS], mask: & [[i32; MAX_CHESS]; MAX_CHESS]) -> bool {
	for row in 0..MAX_CHESS {
        for col in 0..MAX_CHESS {
            if chess[row][col]!=MINE_ID && mask[row][col] == 0{
            	return false;
            }
        }
    }
    return true;
}
fn main() {
	let mut chess: [[i32; MAX_CHESS]; MAX_CHESS] = [[0; MAX_CHESS]; MAX_CHESS];
	let mut mask: [[i32; MAX_CHESS]; MAX_CHESS] = [[0; MAX_CHESS]; MAX_CHESS];
	let mut rng = rand::thread_rng();
	for _ in 0..MAX_MINE {
		let mut index: usize = rng.gen();
		index = index % (MAX_CHESS*MAX_CHESS);
		let row = index / MAX_CHESS;
		let col = index % MAX_CHESS;
		println!("Hello, world!, {:?}, {}", row, col);
		chess[row][col] = MINE_ID;
	}
	add_flag(&mut chess);
	show_chess(&chess, &mask);
	while true {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input_str = input.trim();
        let input_ele:Vec<&str>= input_str.split_whitespace().collect();
		let mut input_row: u8 = input_ele[0].as_bytes()[0];
		let mut input_col: u8 = input_ele[1].as_bytes()[0];
		if input_row >= b'a' {
			input_row -= (b'a'-10);
		}else {
			input_row -= b'0';
		}
		if input_col >= b'a' {
			input_col -= (b'a'-10);
		}else {
			input_col -= b'0';
		}
		let input_row = input_row as usize;
		let input_col = input_col as usize;
		println!("{:?}, {:?}", input_row, input_col);
		if mask[input_row][input_col] == 1 {
			continue;
		}
		mask[input_row][input_col] = 1;
		if chess[input_row][input_col] == 0 {
			spread(&mut chess, &mut mask, input_row, input_col);
		}
		show_chess(&chess, &mask);
		if is_victory(&chess, &mask) {
			println!("Suceess!");
			break;
		}
	}
}
