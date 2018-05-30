#[derive(Clone)]
pub struct Board<T> {
    pub board:Vec<Vec<T>>
}

impl<T:Clone> Board<T>{
    //(x:横のマス目,y:縦のマス目)
    pub fn new(x:u16,y:u16,default:T)->Self{
        let mut rows=Vec::new();
        for _ in 0..y{
            let mut row=Vec::new();
            for _ in 0..x{
                row.push(default.clone());
            }
            rows.push(row);
        }
        Board::<T>{board:rows}
    }

    //指定のマスに値をセットする
    pub fn set(&self,x:usize,y:usize,value:T)->Self{
        let mut new_board=self.board.clone();
        new_board[y][x]=value;
        Board::<T>{board:new_board}
    }
}
