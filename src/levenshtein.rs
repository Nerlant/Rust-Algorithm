use std::cmp::min;

pub fn levenshtein(x: &str, y: &str) -> usize {
    let x_len = x.len();
    let y_len = y.len();
    let mut matrix = vec![vec![0; y.len() + 1]; x_len + 1];

    for i in 1..x_len + 1 {
        matrix[i][0] = i;
    }
    for i in 1..y_len + 1 {
        matrix[0][i] = i;
    }

    println!("\n");
    for i in 0..x_len + 1 {
        println!("{:?}", matrix[i]);
    }

    for j in 1..y_len + 1 {
        for i in 1..x_len + 1 {
            let subs_cost;
            if x.as_bytes()[i - 1] == y.as_bytes()[j - 1] {
                subs_cost = 0;
            } else {
                subs_cost = 1;
            }

            matrix[i][j] = min(
                matrix[i - 1][j - 1] + subs_cost,
                min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1),
            );

            println!("\n");
            for i in 0..x_len + 1 {
                println!("{:?}", matrix[i]);
            }
        }
    }

    matrix[x_len][y_len]
}
