use board::Board;

//リバーシのマス目数
const Reversi_Board_Size: u16 = 8;

//リバーシの石の状態
#[derive(Clone, PartialEq)]
pub enum StoneState {
    Empty,
    Black,
    White,
}

//石の数を保持する構造体
#[derive(Debug, Clone, PartialEq)]
pub struct StoneCount {
    pub blackCount: usize,
    pub whiteCount: usize,
}

impl StoneCount {
    pub fn inc_black(&self) -> StoneCount {
        StoneCount {
            blackCount: self.blackCount + 1,
            ..self.clone()
        }
    }

    pub fn inc_white(&self) -> StoneCount {
        StoneCount {
            whiteCount: self.whiteCount + 1,
            ..(*self)
        }
    }
}

impl ToString for StoneState {
    fn to_string(&self) -> String {
        match self {
            StoneState::Empty => " ",
            StoneState::Black => "x",
            StoneState::White => "o",
        }.to_string()
    }
}

#[derive(Clone)]
pub struct ReversiBoard {
    board: Board<StoneState>,
}

impl ToString for ReversiBoard {
    fn to_string(&self) -> String {
        let mut buf = "  | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |\n".to_string();
        for (index, row) in self.board.board.iter().enumerate() {
            buf+="――――――――――――――――――――――――――――――――――\n";
            buf += &(index.to_string() + " | ");
            for value in row {
                buf += &(value.to_string() + " | ");
            }
            buf += "\n";
        }
        buf
    }
}

impl ReversiBoard {
    pub fn new() -> Self {
        let board =
            Board::<StoneState>::new(Reversi_Board_Size, Reversi_Board_Size, StoneState::Empty);
        let board = board.set(3, 3, StoneState::White);
        let board = board.set(3, 4, StoneState::Black);
        let board = board.set(4, 3, StoneState::Black);
        ReversiBoard {
            board: board.set(4, 4, StoneState::White),
        }
    }

    //黒と白の石の数をカウントする。
    pub fn get_total(&self) -> StoneCount {
        let mut count = StoneCount {
            blackCount: 0,
            whiteCount: 0,
        };
        for row in self.board.board.iter() {
            for value in row {
                count = match value {
                    StoneState::Black => count.inc_black(),
                    StoneState::White => count.inc_white(),
                    _ => count,
                };
            }
        }
        count
    }

    //どこかに置けるかどうかを判定する
    pub fn can_put_any(&self, value: &StoneState) -> bool {
        self.board.board.iter().enumerate().any(|(y, row)| {
            row.iter()
                .enumerate()
                .any(|(x, _)| self.can_put(x, y, value.clone()))
        })
    }

    //石を置けるかどうかを判定する
    fn can_put(&self, x: usize, y: usize, value: StoneState) -> bool {
        //ひっくり返す石の場所リスト
        let reverse_list = self.search(x, y, &value);
        (reverse_list.len() != 0 && self.board.get(x, y) == StoneState::Empty)
    }

    //石を置く
    //置けないとこに置こうとしたり、Emptyを置こうとしたらErrを返す
    pub fn put(&self, x: usize, y: usize, value: StoneState) -> Result<ReversiBoard, &str> {
        match &value {
            StoneState::Empty => Result::Err("null pointer exception!"),
            v => {
                //ひっくり返す石の場所リスト
                let reverse_list = self.search(x, y, &v);
                if reverse_list.len() == 0 || self.board.get(x, y) != StoneState::Empty {
                    Result::Err("そこには置けねえよ！")
                } else {
                    let mut new_board = self.board.clone();
                    for reverse in reverse_list {
                        new_board = new_board.set(reverse.0, reverse.1, value.clone())
                    }
                    new_board = new_board.set(x, y, value.clone());
                    Result::Ok(ReversiBoard { board: new_board })
                }
            }
        }
    }
    //そこに石を置いたらどこが引っくり変えるかの情報を返す
    fn search(&self, x: usize, y: usize, value: &StoneState) -> Vec<(usize, usize)> {
        vec![
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ].into_iter()
            .map(|delta| self.search_line(x, y, delta.0, delta.1, &value))
            .fold(vec![], |acc, x| {
                let mut acc = acc.clone();
                let mut x = x.clone();
                acc.append(&mut x);
                acc
            })
    }

    //移動量を与え、その方向の直線状の引っくり変えるかどうかの判定
    fn search_line(
        &self,
        x: usize,
        y: usize,
        delta_x: i16,
        delta_y: i16,
        value: &StoneState,
    ) -> Vec<(usize, usize)> {
        let mut x: usize = ((x as i16) + delta_x) as usize;
        let mut y: usize = ((y as i16) + delta_y) as usize;
        let mut result = Vec::<(usize, usize)>::new();
        loop {
            //境界チェック
            if (y >= self.board.board.len() || x >= self.board.board[y].len()) {
                return Vec::<(usize, usize)>::new();
            }
            match &self.board.board[y][x] {
                StoneState::Empty => {
                    result = Vec::<(usize, usize)>::new();
                    break;
                }
                v if v != value => result.push((x, y)),
                _ => break,
            };
            x = ((x as i16) + delta_x) as usize;
            y = ((y as i16) + delta_y) as usize;
        }
        result
    }
}
