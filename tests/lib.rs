extern crate reversi;
use reversi::reversi_board::ReversiBoard;

#[test]
fn show_test(){
    let hoge=ReversiBoard::new();
    assert_eq!(hoge.to_string(),"eeeeeeee\neeeeeeee\neeeeeeee\neee○×eee\neee×○eee\neeeeeeee\neeeeeeee\neeeeeeee\n");
}



