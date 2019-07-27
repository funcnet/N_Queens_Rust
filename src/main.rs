const CONST_QUEENS_COUNT: i32 = 12;
const CONST_QUEENS_COUNT_USIZE: usize = CONST_QUEENS_COUNT as usize;

fn main() {
    test_eight_queens();
}

fn test_eight_queens() {
    let mut queen_list: [i32; CONST_QUEENS_COUNT_USIZE] = [0; CONST_QUEENS_COUNT_USIZE];

    unsafe {
        put_queen(CONST_QUEENS_COUNT, &mut queen_list, 0);
    }
}

fn check_conflict(queen_list: &mut [i32; CONST_QUEENS_COUNT_USIZE], next_y: i32) -> bool {
    let next_y = next_y as usize;
    let next_y_int = next_y as i32;
    for position_y in 0..next_y {
        let position_y_int = position_y as i32;
        if (queen_list[position_y] - queen_list[next_y]).abs() == (position_y_int - next_y_int).abs() {
            return true;
        }

        if queen_list[position_y] == queen_list[next_y] {
            return true;
        }
    }
    return false;
}

fn join_array(arr: &[i32]) -> String {
    let mut ret: String = String::from("");
    let length = arr.len();
    for i in 0..length - 1 {
        ret = [ret, arr[i].to_string(), String::from(", ")].concat();
    }
    ret = [ret, arr[length - 1].to_string()].concat();

    return ret;
}

static mut COUNT: i32 = 0;
unsafe fn put_queen(queens_count: i32, queen_list: &mut [i32; CONST_QUEENS_COUNT_USIZE], mut next_y: i32) {
    let next_y_usize = next_y as usize;
    for i in 0..queens_count {
        queen_list[next_y_usize] = i;
        if check_conflict(queen_list, next_y) == false {
            next_y = next_y + 1;
            if next_y < queens_count {
                put_queen(queens_count, queen_list, next_y)
            } else {
                COUNT = COUNT + 1;
                println!("{}: {}", COUNT, join_array(queen_list))
            }
            next_y = next_y - 1;
        }
    }
}
