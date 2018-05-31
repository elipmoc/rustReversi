extern crate reversi;
use reversi::reversi_board::ReversiBoard;
use reversi::reversi_board::StoneCount;

#[test]
fn show_test() {
    let hoge = ReversiBoard::new();
    assert_eq!(hoge.to_string(),
"  | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |\n\
――――――――――――――――――――――――――――――――――\n\
0 |   |   |   |   |   |   |   |   | \n\
――――――――――――――――――――――――――――――――――\n\
1 |   |   |   |   |   |   |   |   | \n\
――――――――――――――――――――――――――――――――――\n\
2 |   |   |   |   |   |   |   |   | \n\
――――――――――――――――――――――――――――――――――\n\
3 |   |   |   | ○ | × |   |   |   | \n\
――――――――――――――――――――――――――――――――――\n\
4 |   |   |   | × | ○ |   |   |   | \n\
――――――――――――――――――――――――――――――――――\n\
5 |   |   |   |   |   |   |   |   | \n\
――――――――――――――――――――――――――――――――――\n\
6 |   |   |   |   |   |   |   |   | \n\
――――――――――――――――――――――――――――――――――\n\
7 |   |   |   |   |   |   |   |   | \n");
}

#[test]
fn count_stone_test() {
    let hoge = ReversiBoard::new();
    assert_eq!(
        hoge.get_total(),
        StoneCount {
            blackCount: 2,
            whiteCount: 2
        }
    );
}
