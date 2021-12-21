use std::borrow::Cow;
use std::fmt;
use std::fs;
use std::io;

struct BingoBoard<'a> {
    board: Cow<'a, [u32]>,
    side: usize,
}

impl<'a> fmt::Display for BingoBoard<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ostr = String::new();

        for (i, val) in self.board.iter().enumerate() {
            ostr.push_str(format!("{:2} ", val).as_str());
            if (i % self.side) == (self.side - 1) {
                ostr.push_str("\n");
            }
        }
        write!(f, "{}", ostr)
    }
}

impl<'a> BingoBoard<'a> {
    const BOARD_SIDE: usize = 5;

    fn new() -> BingoBoard<'a> {
        let bingo_board = BingoBoard {
            board: Cow::Owned(vec![0_u32; BingoBoard::BOARD_SIDE * BingoBoard::BOARD_SIDE]),
            side: BingoBoard::BOARD_SIDE,
        };
        bingo_board
    }

    fn initialize_board_numbers(&mut self, vals: &[u32]) {
        for (i, val) in vals.iter().enumerate() {
            self.board.to_mut()[i] = *val;
        }
    }

    /* Return a vector of the rows completed */
    fn check_rows(&self, numbers: &[usize]) -> Vec<usize> {
        let mut rows: Vec<usize> = vec![];
        for (i, row) in self.board.chunks(5).enumerate() {
            let mut cnt = 0;
            for n in row.iter().map(|x| *x as usize) {
                if numbers.contains(&n) {
                    cnt += 1;
                }
            }
            if cnt == BingoBoard::BOARD_SIDE {
                rows.push(i);
            }
        }
        rows
    }

    /* Return a vector of the cols completed */
    fn check_cols(&self, numbers: &[usize]) -> Vec<usize> {
        let mut cols: Vec<usize> = vec![];
        for j in 0..5 {
            let mut a = self.board.iter();
            for _ in 0..j {
                a.next();
            }
            let mut cnt = 0;
            for val in a.step_by(BingoBoard::BOARD_SIDE).map(|x| *x as usize) {
                if numbers.contains(&val) {
                    cnt += 1;
                    if cnt == BingoBoard::BOARD_SIDE {
                        cols.push(j);
                        break;
                    }
                }
            }
        }
        cols
    }

    fn sum_of_unmarked(&self, numbers: &[usize]) -> u32 {
        let mut sum = 0;
        for val in self.board.iter().map(|x| *x as usize) {
            if numbers.contains(&val) {
                continue;
            }
            sum += val;
        }
        sum as u32
    }
}

fn file_reader<I: io::BufRead>(reader: I) -> (Vec<usize>, Vec<u32>) {
    let mut input_numbers = vec![];
    let mut board_numbers = vec![];

    for (i, line) in reader.lines().enumerate() {
        if i == 0 {
            input_numbers = line
                .unwrap()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
        } else {
            let tmp_str = line.unwrap();
            if tmp_str.trim().eq("") {
                continue;
            } else {
                board_numbers.extend(
                    tmp_str
                        .split_whitespace()
                        .map(|x| x.trim().parse::<u32>().unwrap())
                        .collect::<Vec<u32>>(),
                );
            }
        }
    }

    (input_numbers, board_numbers)
}

fn make_boards(numbers: &[u32]) -> Vec<BingoBoard> {
    let mut boards = Vec::<BingoBoard>::new();
    for window in numbers.chunks(BingoBoard::BOARD_SIDE * BingoBoard::BOARD_SIDE) {
        let mut b = BingoBoard::new();
        b.initialize_board_numbers(&window);
        boards.push(b);
    }
    boards
}

fn find_winner_board(input_numbers: &[usize], boards: &Vec<BingoBoard>) -> (usize, usize) {
    let mut total_input_numbers = 0;
    let mut bingo_board_index = 0;

    'outer_loop: for j in 6..=input_numbers.len() {
        total_input_numbers = j;
        for (i, board) in boards.iter().enumerate() {
            let rows = board.check_rows(&input_numbers[0..j]);

            if rows.len() > 0 {
                bingo_board_index = i;
                break 'outer_loop;
            }
            let cols = board.check_cols(&input_numbers[0..j]);
            if cols.len() > 0 {
                bingo_board_index = i;
                break 'outer_loop;
            }
        }
    }
    (total_input_numbers, bingo_board_index)
}

/* Return a vector of index of last winner boards and the number of input numbers required to win */
fn find_last_winner_board(input_numbers: &[usize], boards: &Vec<BingoBoard>) -> (usize, usize) {
    let mut last_winner_board = 0;
    let mut total_input_numbers = 0;

    for (i, board) in boards.iter().enumerate() {
        for j in 5..input_numbers.len() {
            let rows = board.check_rows(&input_numbers[0..=j]);
            let cols = board.check_cols(&input_numbers[0..=j]);
            if (rows.len() + cols.len()) > 0 {
                if total_input_numbers < j {
                    total_input_numbers = j;
                    last_winner_board = i
                }
                break;
            }
        }
    }
    (total_input_numbers + 1, last_winner_board)
}

fn main() {
    let file_name = "input04.txt";
    let f = fs::File::open(file_name).unwrap();
    let reader = io::BufReader::new(f);

    let (input_numbers, board_numbers) = file_reader(reader);
    let boards = make_boards(&board_numbers);
    let (total_input_numbers, winner) = find_winner_board(input_numbers.as_slice(), &boards);
    let sum = boards[winner].sum_of_unmarked(&input_numbers[0..total_input_numbers]);

    println!("Winner board; {}", winner);
    println!("last number; {}", input_numbers[total_input_numbers - 1]);
    println!("{}", boards[winner]);
    println!(
        "Unchecked product {}*{}: {}",
        input_numbers[total_input_numbers - 1],
        sum,
        input_numbers[total_input_numbers - 1] as u32 * sum
    );
    println!("----");
    let (total_input_numbers, last_winner) =
        find_last_winner_board(input_numbers.as_slice(), &boards);
    let sum = boards[last_winner].sum_of_unmarked(&input_numbers[0..total_input_numbers]);
    println!("Lat winner board; {}", last_winner);
    println!("last number; {}", input_numbers[total_input_numbers - 1]);
    println!("{}", boards[last_winner]);
    println!(
        "Unchecked product {}*{}: {}",
        input_numbers[total_input_numbers - 1],
        sum,
        input_numbers[total_input_numbers - 1] as u32 * sum
    );
}

#[cfg(test)]
mod test {
    use super::*;

    const F: &'static [u8] =
        b"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n
\n
22 13 17 11  0\n
 8  2 23  4 24\n
21  9 14 16  7\n
 6 10  3 18  5\n
 1 12 20 15 19\n
\n
 3 15  0  2 22\n
 9 18 13 17  5\n
19  8  7 25 23\n
20 11 10 24  4\n
14 21 16 12  6\n
\n
14 21 17 24  4\n
10 16 15  9 19\n
18  8 23 26 20\n
22 11 13  6  5\n
2  0 12  3  7" as &[u8];

    #[test]
    fn integration_test_solution_1() {
        let reader = io::BufReader::new(F);

        let (input_numbers, board_numbers) = file_reader(reader);
        let boards = make_boards(&board_numbers);

        let (total_input_numbers, winner) = find_winner_board(input_numbers.as_slice(), &boards);
        let sum = boards[winner].sum_of_unmarked(&input_numbers[0..total_input_numbers]);

        assert_eq!(2, winner);
        assert_eq!(188, sum);
        assert_eq!(12, total_input_numbers);
        assert_eq!(24, input_numbers[total_input_numbers - 1]);
        assert_eq!(
            188 * 24,
            sum * input_numbers[total_input_numbers - 1] as u32
        );
    }

    #[test]
    fn test_find_last_winner_board() {
        let reader = io::BufReader::new(F);

        let (input_numbers, board_numbers) = file_reader(reader);
        let boards = make_boards(&board_numbers);

        let (total_input_numbers, last_winner) =
            find_last_winner_board(input_numbers.as_slice(), &boards);
        let sum = boards[last_winner].sum_of_unmarked(&input_numbers[0..total_input_numbers]);

        assert_eq!(1, last_winner);
        assert_eq!(148, sum);
        assert_eq!(15, total_input_numbers);
        assert_eq!(13, input_numbers[total_input_numbers - 1]);
        assert_eq!(
            148 * 13,
            sum * input_numbers[total_input_numbers - 1] as u32
        );
    }

    #[test]
    fn test_find_winner_board() {
        let mut board = BingoBoard::new();
        board.initialize_board_numbers(&[
            14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3,
            7,
        ]);

        let boards = vec![board];
        let input_numbers = &[7usize, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24];

        let (total_input_numbers, winner_board_index) = find_winner_board(input_numbers, &boards);
        assert_eq!(0, winner_board_index);
        assert_eq!(input_numbers.len(), total_input_numbers);
        assert_eq!(24, input_numbers[total_input_numbers - 1]);
    }

    #[test]
    fn test_bingoboard_initializeboardnumbers() {
        let mut board = BingoBoard::new();
        board.initialize_board_numbers(&(0..25).collect::<Vec<u32>>());
        for i in 0usize..25 {
            assert_eq!(i as u32, board.board[i]);
        }
    }

    #[test]
    fn test_file_reader() {
        let reader = io::BufReader::new(F);
        let (number_sequence, boards) = file_reader(reader);
        assert_eq!(27, number_sequence.len());
        assert_eq!(7, number_sequence[0]);
        assert_eq!(4, number_sequence[1]);
        assert_eq!(1, number_sequence[number_sequence.len() - 1]);

        assert_eq!(75, boards.len());

        assert_eq!(22, boards[0]);
        assert_eq!(19, boards[24]);

        assert_eq!(3, boards[25]);
        assert_eq!(6, boards[49]);

        assert_eq!(14, boards[50]);
        assert_eq!(7, boards[74]);
    }

    #[test]
    fn test_makeboards() {
        let v: Vec<u32> = (0u32..75).collect();

        let boards = make_boards(&v);

        assert_eq!(3, boards.len());

        assert_eq!(0, boards[0].board[0]);
        assert_eq!(24, boards[0].board[24]);

        assert_eq!(50, boards[2].board[0]);
        assert_eq!(74, boards[2].board[24]);
    }

    #[test]
    fn test_bingoboard_check_rows() {
        let v: Vec<u32> = (0u32..24).collect();
        let boards = make_boards(&v);

        let rows = boards[0].check_rows(&[0, 1, 2]);
        assert_eq!(0, rows.len());

        let rows = boards[0].check_rows(&[0, 1, 2, 3, 5, 6, 7, 9, 10, 11, 14]);
        assert_eq!(0, rows.len());

        let rows = boards[0].check_rows(&[0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 14]);
        assert_eq!(1, rows.len());
        assert!(rows.contains(&0usize));

        let rows = boards[0].check_rows(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 14]);
        assert_eq!(2, rows.len());
        assert!(rows.contains(&0usize));
        assert!(rows.contains(&1usize));

        let rows =
            boards[0].check_rows(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 14, 24, 23, 22, 21, 20]);
        assert_eq!(3, rows.len());
        assert!(rows.contains(&0usize));
        assert!(rows.contains(&1usize));
        assert!(rows.contains(&4usize));
    }

    #[test]
    fn test_bingoboard_check_cols() {
        let v: Vec<u32> = (0u32..24).collect();
        let boards = make_boards(&v);

        let cols = boards[0].check_cols(&[0, 1, 2]);
        assert_eq!(0, cols.len());

        let cols = boards[0].check_cols(&[0, 1, 5, 2, 10, 15, 20]);
        assert_eq!(1, cols.len());
        assert!(cols.contains(&0));

        let cols = boards[0].check_cols(&[0, 1, 6, 2, 11, 16, 21, 22]);
        assert_eq!(1, cols.len());
        assert!(cols.contains(&1));

        let cols = boards[0].check_cols(&[0, 1, 7, 2, 12, 17, 21, 22, 23]);
        assert_eq!(1, cols.len());
        assert!(cols.contains(&2));

        let cols = boards[0].check_cols(&[0, 3, 8, 2, 13, 18, 21, 22, 23]);
        assert_eq!(1, cols.len());
        assert!(cols.contains(&3));

        let cols = boards[0].check_cols(&[0, 4, 9, 2, 14, 19, 21, 22, 24]);
        assert_eq!(1, cols.len());
        assert!(cols.contains(&4));

        let cols = boards[0].check_cols(&(0..24).collect::<Vec<usize>>());
        assert_eq!(5, cols.len());
        assert!(cols.contains(&0));
    }

    #[test]
    fn test_sum_of_unmarked() {
        let v: Vec<u32> = (0u32..25).collect();
        let boards = make_boards(&v);

        assert_eq!(25 * 12, boards[0].sum_of_unmarked(&[]));
        assert_eq!(25 * 12 - 23, boards[0].sum_of_unmarked(&[23]));

        assert_eq!(25 * 12, boards[0].sum_of_unmarked(&[]));
        assert_eq!(25 * 12 - 24 - 23, boards[0].sum_of_unmarked(&[23, 24]));
    }
}
