//セル
pub struct cell {
    // 座標
    x: i32,
    y: i32,
    // 生死
    value: bool,
}

//盤面
pub struct Board{
    // サイズ
    width: i32,
    height: i32,
    // セルを格納するベクタ
    cells: Vec<cell>,
}

impl Board {
    // 盤面を新規生成する
    pub fn new(width: i32, height: i32) -> Board {
        let mut cells = Vec::new();
        for y in 0..height {
            for x in 0..width {
                cells.push(cell{x: x, y: y, value: false});
            }
        }
        Board{cells: cells, width: width, height: height}
    }

    //指定の座標のセルを取得する
    pub fn get_cell(&self, x: i32, y: i32) -> Option<&cell> {
        if let Some(cell) = self.cells.get( (x + y*self.width) as usize) {  
            Some(&cell)
        }
        else{
            None
        }
    }

    // 1ステップ進める
    pub fn step(){

    }
}