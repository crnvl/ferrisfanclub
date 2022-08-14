fn main() {
    let arr = [1, 2, 3, 4, 5];
    print_arr_range(&arr, 0, 3);
}

fn print_arr_range(arr: &[i32], start: usize, end: usize) {
    for i in start..end {
        println!("{}", arr[i]);
    }
}
