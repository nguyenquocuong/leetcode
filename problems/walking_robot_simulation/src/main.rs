struct Solution;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let mut direction = (0, 1);
        let mut position = (0, 0);
        let mut max_distance = 0;
        let hs: std::collections::HashSet<(i32, i32)> = obstacles
            .iter()
            .map(|obstacle| (obstacle[0], obstacle[1]))
            .collect();

        for command in commands {
            match command {
                _ if command.is_positive() => {
                    for _ in 1..=command {
                        let next = (position.0 + direction.0, position.1 + direction.1);
                        if hs.contains(&next) {
                            break;
                        } else {
                            position = next;
                            max_distance =
                                max_distance.max(position.0 * position.0 + position.1 * position.1);
                        }
                    }
                }
                -1 => {
                    direction = match direction {
                        (0, 1) => (1, 0),
                        (1, 0) => (0, -1),
                        (0, -1) => (-1, 0),
                        (-1, 0) => (0, 1),
                        _ => unreachable!(),
                    }
                }
                -2 => {
                    direction = match direction {
                        (0, 1) => (-1, 0),
                        (-1, 0) => (0, -1),
                        (0, -1) => (1, 0),
                        (1, 0) => (0, 1),
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }

        max_distance
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        assert_eq!(Solution::robot_sim(vec![4, -1, 3], vec![]), 25);
    }

    #[test]
    fn testcase_2() {
        assert_eq!(
            Solution::robot_sim(vec![4, -1, 4, -2, 4], vec![vec![2, 4]]),
            65
        );
    }

    #[test]
    fn testcase_3() {
        assert_eq!(Solution::robot_sim(vec![6, -1, -1, 6], vec![]), 36);
    }

    #[test]
    fn testcase_4() {
        assert_eq!(
            Solution::robot_sim(
                vec![-2, 8, 3, 7, -1],
                vec![
                    vec![-4, -1],
                    vec![1, -1],
                    vec![1, 4],
                    vec![5, 0],
                    vec![4, 5],
                    vec![-2, -1],
                    vec![2, -5],
                    vec![5, 1],
                    vec![-3, -1],
                    vec![5, -3],
                ]
            ),
            324
        );
    }

    #[test]
    fn testcase_5() {
        assert_eq!(
            Solution::robot_sim(
                vec![
                    1, 2, -2, 5, -1, -2, -1, 8, 3, -1, 9, 4, -2, 3, 2, 4, 3, 9, 2, -1, -1, -2, 1,
                    3, -2, 4, 1, 4, -1, 1, 9, -1, -2, 5, -1, 5, 5, -2, 6, 6, 7, 7, 2, 8, 9, -1, 7,
                    4, 6, 9, 9, 9, -1, 5, 1, 3, 3, -1, 5, 9, 7, 4, 8, -1, -2, 1, 3, 2, 9, 3, -1,
                    -2, 8, 8, 7, 5, -2, 6, 8, 4, 6, 2, 7, 2, -1, 7, -2, 3, 3, 2, -2, 6, 9, 8, 1,
                    -2, -1, 1, 4, 7
                ],
                vec![
                    vec![-57, -58],
                    vec![-72, 91],
                    vec![-55, 35],
                    vec![-20, 29],
                    vec![51, 70],
                    vec![-61, 88],
                    vec![-62, 99],
                    vec![52, 17],
                    vec![-75, -32],
                    vec![91, -22],
                    vec![54, 33],
                    vec![-45, -59],
                    vec![47, -48],
                    vec![53, -98],
                    vec![-91, 83],
                    vec![81, 12],
                    vec![-34, -90],
                    vec![-79, -82],
                    vec![-15, -86],
                    vec![-24, 66],
                    vec![-35, 35],
                    vec![3, 31],
                    vec![87, 93],
                    vec![2, -19],
                    vec![87, -93],
                    vec![24, -10],
                    vec![84, -53],
                    vec![86, 87],
                    vec![-88, -18],
                    vec![-51, 89],
                    vec![96, 66],
                    vec![-77, -94],
                    vec![-39, -1],
                    vec![89, 51],
                    vec![-23, -72],
                    vec![27, 24],
                    vec![53, -80],
                    vec![52, -33],
                    vec![32, 4],
                    vec![78, -55],
                    vec![-25, 18],
                    vec![-23, 47],
                    vec![79, -5],
                    vec![-23, -22],
                    vec![14, -25],
                    vec![-11, 69],
                    vec![63, 36],
                    vec![35, -99],
                    vec![-24, 82],
                    vec![-29, -98],
                    vec![-50, -70],
                    vec![72, 95],
                    vec![80, 80],
                    vec![-68, -40],
                    vec![65, 70],
                    vec![-92, 78],
                    vec![-45, -63],
                    vec![1, 34],
                    vec![81, 50],
                    vec![14, 91],
                    vec![-77, -54],
                    vec![13, -88],
                    vec![24, 37],
                    vec![-12, 59],
                    vec![-48, -62],
                    vec![57, -22],
                    vec![-8, 85],
                    vec![48, 71],
                    vec![12, 1],
                    vec![-20, 36],
                    vec![-32, -14],
                    vec![39, 46],
                    vec![-41, 75],
                    vec![13, -23],
                    vec![98, 10],
                    vec![-88, 64],
                    vec![50, 37],
                    vec![-95, -32],
                    vec![46, -91],
                    vec![10, 79],
                    vec![-11, 43],
                    vec![-94, 98],
                    vec![79, 42],
                    vec![51, 71],
                    vec![4, -30],
                    vec![2, 74],
                    vec![4, 10],
                    vec![61, 98],
                    vec![57, 98],
                    vec![46, 43],
                    vec![-16, 72],
                    vec![53, -69],
                    vec![54, -96],
                    vec![22, 0],
                    vec![-7, 92],
                    vec![-69, 80],
                    vec![68, -73],
                    vec![-24, -92],
                    vec![-21, 82],
                    vec![32, -1],
                    vec![-6, 16],
                    vec![15, -29],
                    vec![70, -66],
                    vec![-85, 80],
                    vec![50, -3],
                    vec![6, 13],
                    vec![-30, -98],
                    vec![-30, 59],
                    vec![-67, 40],
                    vec![17, 72],
                    vec![79, 82],
                    vec![89, -100],
                    vec![2, 79],
                    vec![-95, -46],
                    vec![17, 68],
                    vec![-46, 81],
                    vec![-5, -57],
                    vec![7, 58],
                    vec![-42, 68],
                    vec![19, -95],
                    vec![-17, -76],
                    vec![81, -86],
                    vec![79, 78],
                    vec![-82, -67],
                    vec![6, 0],
                    vec![35, -16],
                    vec![98, 83],
                    vec![-81, 100],
                    vec![-11, 46],
                    vec![-21, -38],
                    vec![-30, -41],
                    vec![86, 18],
                    vec![-68, 6],
                    vec![80, 75],
                    vec![-96, -44],
                    vec![-19, 66],
                    vec![21, 84],
                    vec![-56, -64],
                    vec![39, -15],
                    vec![0, 45],
                    vec![-81, -54],
                    vec![-66, -93],
                    vec![-4, 2],
                    vec![-42, -67],
                    vec![-15, -33],
                    vec![1, -32],
                    vec![-74, -24],
                    vec![7, 18],
                    vec![-62, 84],
                    vec![19, 61],
                    vec![39, 79],
                    vec![60, -98],
                    vec![-76, 45],
                    vec![58, -98],
                    vec![33, 26],
                    vec![-74, -95],
                    vec![22, 30],
                    vec![-68, -62],
                    vec![-59, 4],
                    vec![-62, 35],
                    vec![-78, 80],
                    vec![-82, 54],
                    vec![-42, 81],
                    vec![56, -15],
                    vec![32, -19],
                    vec![34, 93],
                    vec![57, -100],
                    vec![-1, -87],
                    vec![68, -26],
                    vec![18, 86],
                    vec![-55, -19],
                    vec![-68, -99],
                    vec![-9, 47],
                    vec![24, 94],
                    vec![92, 97],
                    vec![5, 67],
                    vec![97, -71],
                    vec![63, -57],
                    vec![-52, -14],
                    vec![-86, -78],
                    vec![-17, 92],
                    vec![-61, -83],
                    vec![-84, -10],
                    vec![20, 13],
                    vec![-68, -47],
                    vec![7, 28],
                    vec![66, 89],
                    vec![-41, -17],
                    vec![-14, -46],
                    vec![-72, -91],
                    vec![4, 52],
                    vec![-17, -59],
                    vec![-85, -46],
                    vec![-94, -23],
                    vec![-48, -3],
                    vec![-64, -37],
                    vec![2, 26],
                    vec![76, 88],
                    vec![-8, -46],
                    vec![-19, -68]
                ]
            ),
            5140
        );
    }
}
