pub fn annotate(minefield: &[&str]) -> Vec<String> {
    const BOMB: i32 = -1;

    let rows = minefield.len();
    if rows == 0 {
        return Vec::new();
    }
    let cols = minefield[0].len();

    let mut num_field = Vec::<Vec<i32>>::with_capacity(rows);
    for i in 0..rows {
        let mut row = Vec::<i32>::with_capacity(cols);
        for j in 0..cols {
            if minefield[i].chars().nth(j) == Some('*') {
                row.push(BOMB);
            } else {
                row.push(0);
            }
        }
        num_field.push(row);
    }

    for i in 0..rows {
        for j in 0..cols {
            if num_field[i][j] == BOMB {
                continue;
            }

            let mut c = 0;
            // top left
            if i >= 1 && j >= 1 && num_field[i - 1][j - 1] == BOMB {
                c += 1;
            }
            // top center
            if i >= 1 && num_field[i - 1][j] == BOMB {
                c += 1;
            }
            // top right
            if i >= 1 && j < cols - 1 && num_field[i - 1][j + 1] == BOMB {
                c += 1;
            }
            // middle left
            if j >= 1 && num_field[i][j - 1] == BOMB {
                c += 1;
            }
            // middle right
            if j < cols - 1 && num_field[i][j + 1] == BOMB {
                c += 1;
            }
            // bottom left
            if i < rows - 1 && j >= 1 && num_field[i + 1][j - 1] == BOMB {
                c += 1;
            }
            // bottom center
            if i < rows - 1 && num_field[i + 1][j] == BOMB {
                c += 1;
            }
            // bottom right
            if i < rows - 1 && j < cols - 1 && num_field[i + 1][j + 1] == BOMB {
                c += 1;
            }
            num_field[i][j] = c;
        }
    }

    let mut result = Vec::<String>::with_capacity(rows);
    for i in 0..rows {
        let mut row = "".to_string();
        for j in 0..cols {
            let n = num_field[i][j];
            if n == BOMB {
                row.push('*');
            } else if n == 0 {
                row.push(' ');
            } else {
                row.push_str(&num_field[i][j].to_string());
            }
        }
        result.push(row);
    }
    result
}
