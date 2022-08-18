fn main() {
    let a:  Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let b:  Vec<i32> = vec![10, 20, 30, 40, 50, 60];

    let c:  Vec<i32> = merge(a, b);
    println!("{:?}", c);
}

fn merge(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut c: Vec<i32> = vec![0; a.len() + b.len()]; 

    let mut index_a = 0;
    let mut index_b = 0;

    for i in 0..c.len() {
        if index_a < a.len() && (index_b == b.len() || a[index_a] < b[index_b]) {
            c[i] = a[index_a];
            index_a += 1;
        } else {
            c[i] = b[index_b];
            index_b += 1;
        }
    }
    return c
}
