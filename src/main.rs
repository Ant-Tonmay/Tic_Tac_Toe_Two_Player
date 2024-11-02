use std::io;
const ALIEN_X: char = 'X';
const ALIEN_O: char = '0';
const BOARD_SIZE: usize = 8;
type Board = Vec<Vec<char>>;

fn init() -> Board {
    let mut board: Board = vec![vec![' '; BOARD_SIZE]; BOARD_SIZE];
    for i in 0..BOARD_SIZE {
        board[0][i] = '■';
        board[7][i] = '■';
    }
    for i in 1..=6 {
        board[i][0] = '■';
        board[i][7] = '■';
    }
    board[1][3] = '0';
    board[1][4] = '1';
    board[1][5] = '2';
    board[3][1] = '0';
    board[4][1] = '1';
    board[5][1] = '2';
    for i in 0..3 {
        for j in 0..3 {
            board[i + 3][j + 3] = '-';
        }
    }
    return board;
}
fn print_board(board: &Board) {
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            print!("{} ", board[i][j]);
        }
        print!("\n");
    }
}
fn ask_cordinates(current_alien: char, board: &Board) -> (usize, usize) {
    loop {
        println!("Alien {} Mark your cell :(row,col)", current_alien);
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        let cord: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse::<usize>().expect("Parse error"))
            .collect();

        if cord.len() == 2 {
            let (row, col): (usize, usize) = (cord[0], cord[1]);
            if row + 3 < 6 && col + 3 < 6 && board[row+3][col+3] == '-' {
                return (row+3, col+3);
            }
        }
        println!("Invalid Input");
    }
}


fn start_game(board: &mut Board) {
    
    let mut current_alien = ALIEN_X;

    loop {
        println!("Currnet Board");
        print_board(&board);
        let(row,col) = ask_cordinates(current_alien, &board);
        board[row][col]=current_alien;
        current_alien = if current_alien == ALIEN_X {
            ALIEN_O
        } else {
            ALIEN_X
        }
    }
}

fn main() {
    println!("
██╗░░██╗░█████╗░████████╗░█████╗░  ██╗░░██╗██╗░░░██╗████████╗██╗
██║░██╔╝██╔══██╗╚══██╔══╝██╔══██╗  ██║░██╔╝██║░░░██║╚══██╔══╝██║
█████═╝░███████║░░░██║░░░███████║  █████═╝░██║░░░██║░░░██║░░░██║
██╔═██╗░██╔══██║░░░██║░░░██╔══██║  ██╔═██╗░██║░░░██║░░░██║░░░██║
██║░╚██╗██║░░██║░░░██║░░░██║░░██║  ██║░╚██╗╚██████╔╝░░░██║░░░██║
╚═╝░░╚═╝╚═╝░░╚═╝░░░╚═╝░░░╚═╝░░╚═╝  ╚═╝░░╚═╝░╚═════╝░░░░╚═╝░░░╚═╝");
    println!();
    println!("
█▀█ █░█ █░░ █▀▀ █▀
█▀▄ █▄█ █▄▄ ██▄ ▄█");


    println!("1. 𝕰𝖆𝖈𝖍 𝖕𝖑𝖆𝖞𝖊𝖗 𝖜𝖎𝖑𝖑 𝖒𝖆𝖗𝖐 𝖙𝖍𝖊 𝖈𝖊𝖑𝖑 𝖚𝖘𝖎𝖓𝖌 (𝖗𝖔𝖜,𝖈𝖔𝖑) 𝖊𝖌 - (1,1)");
    println!("2. 𝕿𝖔 𝕾𝖙𝖔𝖕 𝖙𝖍𝖊 𝖌𝖆𝖒𝖊 𝖕𝖗𝖊𝖘𝖘 𝖈𝖙𝖗𝖑+𝖈");

    
    let mut board: Board = init();
    
    print_board(&board);
    start_game(&mut board);
}
