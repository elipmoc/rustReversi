use reversi_board::{ReversiBoard,StoneState};
use std::io;


enum PlayerType{
    White,
    Black
}

impl ToString for PlayerType{
    fn to_string(&self)->String{
        match self{
            PlayerType::White=>"白".to_string(),
            PlayerType::Black=>"黒".to_string()
        }
    }
}

impl PlayerType{
    fn to_StoneState(&self)->StoneState{
        match self{
            PlayerType::White=>StoneState::White,
            PlayerType::Black=>StoneState::Black
        }
    }
    fn reverse(&self)->Self{
        match self{
            PlayerType::White => PlayerType::Black,
            _=>PlayerType::White
        }
    }
}

pub fn game_start(){
    println!("{}","リバーシゲーム" );
    turn(ReversiBoard::new(),PlayerType::White);
}

fn turn(reversi_board:ReversiBoard,player:PlayerType){
    println!("{}",reversi_board.to_string() ); 
    println!("{}を置きたいマス目を選んでね",player.to_string());
    println!("{}","横の座標を入力してね");
    let mut buf=String::new();
    io::stdin().read_line(&mut buf).expect("error");
    let x=buf.trim().parse::<i32>().unwrap() as usize;
    println!("{}","縦の座標を入力してね");
    buf="".to_string();
    io::stdin().read_line(&mut buf).expect("error");
    let y=buf.trim().parse::<i32>().unwrap() as usize;


    match &reversi_board.put(x,y,player.to_StoneState()) {
        Result::Err(msg)=>{
            println!("{}",msg);
            turn(reversi_board.clone(),player);
        }
        Result::Ok(ref new_reversi_board)=>{
            turn((*new_reversi_board).clone(),player.reverse());
        }
    };
}