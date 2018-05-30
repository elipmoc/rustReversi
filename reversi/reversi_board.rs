use board::{Board};

//リバーシのマス目数
const Reversi_Board_Size:u16=8;

//リバーシの石の状態
#[derive(Clone,PartialEq)]
pub enum StoneState{
    Empty,
    Black,
    White
}

impl ToString for StoneState{
    fn to_string(&self)->String{
        match self{
            StoneState::Empty=>"e",
            StoneState::Black=>"×",
            StoneState::White=>"○"
        }.to_string()
    }
}

#[derive(Clone)]
pub struct ReversiBoard{
    board:Board<StoneState>
}

impl ToString for ReversiBoard{
    fn to_string(&self)->String{
        let mut buf="".to_string();
        for row in &self.board.board{
            for value in row{
                buf+=&value.to_string();
            }
            buf+="\n";
        }
        buf
    }
}

impl ReversiBoard{
    pub fn new()->Self{
        let board=
            Board::<StoneState>::new(Reversi_Board_Size,Reversi_Board_Size,StoneState::Empty);
        let board = board.set(3,3,StoneState::White);
        let board = board.set(3,4,StoneState::Black);
        let board = board.set(4,3,StoneState::Black);
        ReversiBoard{board: board.set(4,4,StoneState::White)}
    }


    //石を置く
    //置けないとこに置こうとしたり、Emptyを置こうとしたらErrを返す
    pub fn put(&self,x:usize,y:usize,value:StoneState)->Result<ReversiBoard,&str>{
        match &value{
            StoneState::Empty=>Result::Err("null pointer exception!"),
            v=>{
                //ひっくり返す石の場所リスト
                let reverse_list = self.search(x,y,&v);
                if reverse_list.len()==0{
                    Result::Err("そこには置けねえよ！")
                }
                else{
                    let mut new_board=self.board.clone();                
                    for reverse in reverse_list{
                        new_board=new_board.set(reverse.0,reverse.1,value.clone())
                    }
                    new_board=new_board.set(x,y,value.clone());
                    Result::Ok(ReversiBoard{board: new_board})
                }
            }
        }
    }
    //そこに石を置いたらどこが引っくり変えるかの情報を返す
    fn search(&self,x:usize,y:usize,value:&StoneState)->Vec<(usize,usize)>{
        vec![(0,1),(1,0),(0,-1),(-1,0),(1,1),(-1,-1),(1,-1),(-1,1)].into_iter().map(|delta|{
            self.search_line(x,y,delta.0,delta.1,&value)
        }).fold(vec![],|acc,x|{
            let mut acc=acc.clone();
            let mut x=x.clone();
            acc.append(&mut x);
            acc
        })
    }

    //移動量を与え、その方向の直線状の引っくり変えるかどうかの判定
    fn search_line(&self,x:usize,y:usize,delta_x:i16,delta_y:i16,value:&StoneState)->Vec<(usize,usize)>{
        let mut x:usize=((x as i16)+delta_x) as usize;
        let mut y:usize=((y as i16)+delta_y) as usize;
        let mut result = Vec::<(usize,usize)>::new();
        loop{
            //境界チェック
            if(y>=self.board.board.len() || x>=self.board.board[y].len()){
                return Vec::<(usize,usize)>::new();
            }
            match &self.board.board[y][x] {
                StoneState::Empty=>{result=Vec::<(usize,usize)>::new();break;},
                v if v!=value => result.push((x,y)), 
                _ => break 
            };
            x=((x as i16)+delta_x) as usize;
            y=((y as i16)+delta_y) as usize;
        }
        result
    }
}