use std::io;
use std::io::Write;

fn main() {
    let mut board = init();
    print_board(&board);
    let mut b_num = 0;
    let mut w_num = 0;
    //true: black, flase: white
    let mut turn = true;
    loop {
        if can_flip(&mut board, turn) {
            movement(&mut board, turn);
        } else {
            if turn {
                println!("B player has no valid move.");
            } else {
                println!("W player has no valid move.");
            }
            if !can_flip(&mut board, !turn) {
                if turn {
                    println!("W player has no valid move.");
                } else {
                    println!("B player has no valid move.");
                }
                for i in 1..9 {
                    for j in 2..10 {
                        if board[i][j] == 'B' {
                            b_num += 1;
                        } else if board[i][j] == 'W' {
                            w_num += 1;
                        }
                    }
                }
                if b_num > w_num {
                    println!("Black wins by {} points!", b_num - w_num);
                } else if w_num > b_num {
                    println!("White wins by {} points!", w_num - b_num);
                } else {
                    println!("Draw!");
                }
                break;
            }
        }
        turn = !turn;
    }
}

fn flip(board: &mut [[char; 10]; 9], x: usize, y: usize, turn: bool, change: bool) -> bool {
    //check if the piece can be placed here
    if !in_border(x, y) || board[x][y] != '.' {
        return false;
    }
    let mut res = false;
    let dx: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
    let dy: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];

    let my_color = if turn { 'B' } else { 'W' };
    let opp_color = if turn { 'W' } else { 'B' };
    for i in 0..8 {
        let mut a: usize = (x as i32 + dx[i]) as usize;
        let mut b: usize = (y as i32 + dy[i]) as usize;
        while in_border(a, b) && board[a][b] == opp_color {
            a = (a as i32 + dx[i]) as usize;
            b = (b as i32 + dy[i]) as usize;
        }
        //found possible direction, flip opponent's color
        if in_border(a, b) && board[a][b] == my_color {
            while !(x == a && y == b) {
                if board[a][b] != my_color {
                    res = true;
                    if change {
                        board[a][b] = my_color;
                    }
                }
                a = (a as i32 - dx[i]) as usize;
                b = (b as i32 - dy[i]) as usize;
            }
        }
    }
    if res && change {
        board[x][y] = my_color;
    }
    res
}

fn in_border(x: usize, y: usize) -> bool {
    if x < 1 || x > 8 || y < 2 || y > 9 {
        false
    } else {
        true
    }
}

fn movement(board: &mut [[char; 10]; 9], turn: bool) {
    let mut input = String::new();
    loop {
        if turn {
            print!("Enter move for colour B (RowCol): ");
        } else {
            print!("Enter move for colour W (RowCol): ");
        }
        io::stdout().flush().expect("Failed to flush stdout.");
        input.clear();
        io::stdin().read_line(&mut input).expect("unable to read");
        let s = input.trim();
        //check len
        if s.len() != 2 {
            println!("Invalid move. Try again.");
            print_board(board);
            continue;
        }
        let bytes = s.as_bytes();
        //check char
        let row_char = bytes[0] as char;
        let col_char = bytes[1] as char;
        if !('a'..='h').contains(&row_char) || !('a'..='h').contains(&col_char) {
            println!("Invalid move. Try again.");
            print_board(board);
            continue;
        }
        let x = (bytes[0] - b'a' + 1) as usize;
        let y = (bytes[1] - b'a' + 2) as usize;

        if flip(board, x, y, turn, true) {
            print_board(board);
            break;
        }
        println!("Invalid move. Try again.");
        print_board(board);
    }
}

fn can_flip(board: &mut [[char; 10]; 9], turn: bool) -> bool {
    for i in 1..9 {
        for j in 2..10 {
            if flip(board, i, j, turn, false) {
                return true;
            }
        }
    }
    false
}

fn init() -> [[char; 10]; 9] {
    let mut board = [['.'; 10]; 9];
    board[0] = [' ', ' ', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    for i in 1..9 {
        board[i][0] = board[0][i + 1];
        board[i][1] = ' ';
    }
    board[4][5] = 'W';
    board[4][6] = 'B';
    board[5][6] = 'W';
    board[5][5] = 'B';
    board
}

fn print_board(board: &[[char; 10]; 9]) {
    for row in board {
        for &item in row {
            print!("{}", item);
        }
        println!();
    }
}
