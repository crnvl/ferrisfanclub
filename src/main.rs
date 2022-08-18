use rand::{thread_rng, Rng};

fn main() {
    let mut a: Vec<i32> = vec![0; 2000000];
    let mut b: Vec<i32> = vec![0; 2000000];

    for i in 0..a.len() {
        a[i] = thread_rng().gen_range(0..100);
    }

    for i in 0..b.len() {
        b[i] = thread_rng().gen_range(0..100);
    }

    a.sort();
    b.sort();

    let c: Vec<i32> = merge(a, b);
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
    return c;
}
