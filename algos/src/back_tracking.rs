#[allow(dead_code)]
// 递归+回溯 O(1)
// https://leetcode.cn/problems/sudoku-solver/solutions/2431201/rustdi-gui-hui-su-0msshuang-100tai-jiao-zjnww/
fn sudoku(nums: &mut Vec<Vec<char>>) {
      // 这个对象每个元素表示一行，把它看成二进制，每一位表示一个数字
        // 比如100000001表示该行只有1和9，其他空缺
        let mut line = vec![0; 9];  // 行字典
        let mut row = vec![0; 9];  // 列字典
        let mut square = vec![vec![0; 3]; 3];  // 9宫格字典
        let mut unsolved = Vec::new();
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != '.' {
                    // bin的末位代表1，其他位置通过移位实现
                    let bin = 1 << (board[i][j] as u8 - '1' as u8);
                    // 或操作把bin上为1的位置，赋给line[i]对应位
                    line[i] |= bin;
                    row[j] |= bin;
                    square[i/3][j/3] |= bin;
                } else {
                    // 存储待解决的位置，降低迭代次数
                    unsolved.push((i, j));
                }
            }
        }
        // 这段代码并非没有意义，它可以降低递归规模，但是这个问题本身空间不是特别大，效果不明显
        // 这里每次循环可以确定那些只有唯一值的单元，对于比较简单的数独题，这样就可以求解了
        // 如果有代码看不明白的，可以把这段注释放开执行，打印内容清楚记录着unsolved的变化情况
        // loop {
        //     print!("\n----{}----", unsolved.len());
        //     let mut to_remove: Vec<usize> = Vec::new();
        //     for (k, &(i,j)) in unsolved.iter().enumerate() {
        //         // 与运算，再取反，值为1的位置代表三个方向共同缺少的值
        //         let mut bin = 0b111111111 ^ (line[i] | row[j] | square[i/3][j/3]);
        //         print!("\n{:b},({},{})", bin, i, j);
        //         // 如果bin有且仅有一位为1
        //         if bin != 0 && (bin & (bin - 1)) == 0 {
        //             print!(",{}", ('1' as u8 + bin.trailing_zeros() as u8) as char);
        //             line[i] |= bin;
        //             row[j] |= bin;
        //             square[i/3][j/3] |= bin;
        //             board[i][j] = ('1' as u8 + bin.trailing_zeros() as u8) as char;
        //             to_remove.push(k);
        //         }
        //     }
        //     if to_remove.len() == 0 {
        //         break
        //     }
        //     for &k in to_remove.iter().rev() {
        //         unsolved.remove(k);
        //     }
        // }
        backtracking(board, &mut unsolved, &mut line, &mut row, &mut square);
    
}
// 参数太多了233，本来想写一个闭包函数的，可是闭包要实现递归相当复杂
pub fn backtracking(board: &mut Vec<Vec<char>>, unsolved: &mut Vec<(usize, usize)>, line: &mut Vec<usize>, row: &mut Vec<usize>, square: &mut Vec<Vec<usize>>) -> bool {
    for &(i,j) in unsolved.iter() {
        // 只处理待解决的问题，这样递归的时候才会往下走（或者可以在递归函数加上一个position参数，强迫往下走）
        if board[i][j] == '.' {
            // 与运算，值为0的位代表三个方向共同缺少的值
            // 再按位取反，值为1的位对应的数字代表可选值
            let mut bin = 0b111111111 ^ (line[i] | row[j] | square[i/3][j/3]);
            while bin != 0 {
                // bin.trailing_zeros()代表bin末尾有多少个0
                board[i][j] = ('1' as u8 + bin.trailing_zeros() as u8) as char;
                // 最末尾的1，比如bin是1010，代表bin可选1或者4，bit的值就是10
                let bit  = 1 << bin.trailing_zeros();
                line[i] |= bit;
                row[j] |=  bit;
                square[i/3][j/3] |= bit;
                
                if backtracking(board, unsolved, line, row, square) {
                    return true;
                }
                // 不行就回溯
                board[i][j] = '.';
                line[i] ^= bit;
                row[j] ^=  bit;
                square[i/3][j/3] ^= bit;
                // 尝试下一个，把最靠后的1删了
                bin -= bit;
            }
            // 都不行那就是不行了
            return false;
        }
    }
    // 所有位置都OK那就OK了
    true
}


#[cfg(test)]
mod back_tracking_tests {
    use super::*;

    #[test]
    fn sudoku() {
        let board = [['5','3','.','.','7','.','.','.','.'],['6','.','.','1','9','5','.','.','.'],['.','9','8','.','.','.','.','6','.'],['8','.','.','.','6','.','.','.','3'],['4','.','.','8','.','3','.','.','1'],['7','.','.','.','2','.','.','.','6'],['.','6','.','.','.','.','2','8','.'],['.','.','.','4','1','9','.','.','5'],['.','.','.','.','8','.','.','7','9']];
        // sudoku(board)
    }
}