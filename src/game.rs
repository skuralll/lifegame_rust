//セル
#[derive(Debug, Clone)]
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

const AROUND_POS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Board {
    // 盤面を新規生成する
    pub fn new(width: i32, height: i32) -> Board {
        let mut cells = Vec::new();
        for y in 0..height {
            for x in 0..width {
                cells.push(cell{x: x, y: y, value: false});
            }
        }
        Self {cells: cells, width: width, height: height}
    }

    // 指定の座標のセルを取得する
    pub fn get_cell(&self, x: i32, y: i32) -> Option<&cell> {
        //両端を繋げないための処理
        if 0 > x && x >= self.width {
            return None;
        }
        //セルが存在していれば返す
        if let Some(cell) = self.cells.get( (x + y*self.width) as usize) {  
            Some(cell)
        }
        else{
            None
        }
    }

    // 1ステップ進める
    pub fn step(&mut self){
        let mut after_cells: Vec<cell> =self.cells.to_vec(); // 処理終了後のVector
        for cell in self.cells.iter(){
            //周りの生存セル数をカウントする
            let mut around_count = 0;
            for pos in AROUND_POS.iter(){
                let x = cell.x + pos.0;
                let y = cell.y + pos.1;
                //セル取得
                if let Some(around_cell) = self.get_cell(x, y){
                    if around_cell.value{
                        around_count += 1;
                    }
                }
            }
            // 生死判定
            if cell.value && (around_count < 2 || around_count > 3) || 
              !cell.value && around_count == 3{
                //生死が変わる場合、aftercell内の該当cellの値を変更する
               if let Some(after_cell) = after_cells.get_mut((cell.x + cell.y*self.width) as usize){
                    after_cell.value = !after_cell.value; // 状態反転
               }
            }
        }
        // cellsをaftercellに置き換える
        self.cells = after_cells; 
    }
}