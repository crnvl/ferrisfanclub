// write a program that rolls a dice ten times.

fn main() {
    const a: [i32; 6] = [1, 2, 3, 4, 5, 6];
    const b: [i32; 6] = [10, 20, 30, 40, 50, 60];

    let c: [i32; a.len() + b.len()] = merge(&a, &b);
    println!("{:?}", c);
}

fn merge(a: &[i32; 6], b: &[i32; 6]) -> [i32; 12] {
    let mut c: [i32; 12] = [0; 12];

    let mut indexA = 0;
    let mut indexB = 0;

    for i in 0..12 {
        if indexA < a.len() && (indexB == b.len() || a[indexA] < b[indexB]) {
            c[i] = a[indexA];
            indexA += 1;
        } else {
            c[i] = b[indexB];
            indexB += 1;
        }
    }
    return c
}
