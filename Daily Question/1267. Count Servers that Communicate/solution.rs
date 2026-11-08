impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut count: i32 = 0;

        let mut rowCounts: Vec<i32> = vec![0; grid.len()];
        let mut colCounts: Vec<i32> = vec![0; grid[0].len()];

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 1 {
                    rowCounts[i] += 1;
                    colCounts[j] += 1;
                }
            }
        }

        for k in 0..grid.len() {
            for l in 0..grid[k].len() {
                if grid[k][l] == 1 {
                    if rowCounts[k] > 1 || colCounts[l] > 1 {
                        count += 1;
                    }
                }
            }
        }

        return count;
    }
}
