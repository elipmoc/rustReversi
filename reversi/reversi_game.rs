use reversi_board::{ReversiBoard, StoneCount, StoneState};
use std::io;

enum PlayerType {
    White,
    Black,
}

impl ToString for PlayerType {
    fn to_string(&self) -> String {
        match self {
            PlayerType::White => "白".to_string(),
            PlayerType::Black => "黒".to_string(),
        }
    }
}

impl PlayerType {
    fn to_StoneState(&self) -> StoneState {
        match self {
            PlayerType::White => StoneState::White,
            PlayerType::Black => StoneState::Black,
        }
    }
    fn reverse(&self) -> Self {
        match self {
            PlayerType::White => PlayerType::Black,
            _ => PlayerType::White,
        }
    }
}

fn read_input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("error");
    match buf.trim().parse::<i32>() {
        Result::Err(_) => read_input(),
        _ => buf,
    }
}

pub fn game_start() {
    println!("{}", "リバーシゲーム");
    turn(ReversiBoard::new(), PlayerType::White);
}

fn turn(reversi_board: ReversiBoard, player: PlayerType) {
    println!("{}", reversi_board.to_string());
    if reversi_board.can_put_any(&player.to_StoneState()) == false {
        if reversi_board.can_put_any(&player.reverse().to_StoneState()) == false {
            println!("{}", "勝負終わり");
            let counts: StoneCount = reversi_board.get_total();
            println!("白:{}黒:{}", counts.whiteCount, counts.blackCount);
            if counts.blackCount > counts.whiteCount {
                println!("{}", "黒の勝ち！")
            } else if counts.blackCount < counts.whiteCount {
                println!("{}", "白の勝ち！")
            } else {
                println!("{}", "引き分け！")
            }
            return ();
        } else {
            println!("{}", "pass");
            return turn(reversi_board, player.reverse());
        }
    };
    println!(
        "{}を置きたいマス目を選んでね",
        player.to_string()
    );
    println!("{}", "横の座標を入力してね");
    let mut buf = read_input();
    let x = buf.trim().parse::<i32>().unwrap() as usize;
    println!("{}", "縦の座標を入力してね");
    buf = read_input();
    let y = buf.trim().parse::<i32>().unwrap() as usize;

    match &reversi_board.put(x, y, player.to_StoneState()) {
        Result::Err(msg) => {
            println!("{}", msg);
            turn(reversi_board.clone(), player);
        }
        Result::Ok(ref new_reversi_board) => {
            turn((*new_reversi_board).clone(), player.reverse());
        }
    };
}
