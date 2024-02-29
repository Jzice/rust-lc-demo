//! 图论算法
//!

/// # 弗洛伊德算法
/// - 求任意两点间的最短路径
pub fn floyd_warshall(graph: Vec<Vec<i32>>) -> (Vec<Vec<i32>>, Vec<Vec<i32>>){
    let n = graph.len();
    let mut distance = graph.clone();   // 每个点到其他点的最短距离
    let mut path = vec![vec![0; n]; n]; // 每个点到其他点最短距离的路径

    // 初始化path
    for i in 0..n {
        for j in 0..n {
            path[i][j] = j as i32;
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if distance[i][k] != -1 && distance[k][j] != -1 {
                    let d = distance[i][k] + distance[k][j];
                    if distance[i][j] == -1 || d < distance[i][j] {
                        distance[i][j] = d;
                        path[i][j] = k as i32;
                    }
                }
            }
        }
    }

    (distance, path)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(floyd_warshall(vec![
                vec![0, 2,-1, 6],
                vec![2, 0, 3, 2],
                vec![-1,3, 0, 2],
                vec![6, 2, 2, 0],
            ]),
            (
                vec![
                    vec![0,2,5,4],
                    vec![2,0,3,2],
                    vec![5,3,0,2],
                    vec![4,2,2,0]
                ],
                vec![
                    vec![0,1,1,1],
                    vec![0,1,2,3],
                    vec![1,1,2,3],
                    vec![1,1,2,3]
                ],
            )
        )
    }
}
