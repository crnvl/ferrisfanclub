// write a program that rolls a dice ten times.

fn main() {
    const A: [i32; 6] = [1, 2, 3, 4, 5, 6];
    const B: [i32; 6] = [10, 20, 30, 40, 50, 60];

    let c: [i32; A.len() + B.len()] = merge(&A, &B);
    println!("{:?}", c);
}

fn merge(a: &[i32; 6], b: &[i32; 6]) -> [i32; 12] {
    let mut c: [i32; 12] = [0; 12];

    let mut index_a = 0;
    let mut index_b = 0;

    for i in 0..12 {
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
