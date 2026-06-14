fn main() {
    let nums = [0, 5, 6, 1, 6, 7, 1, 3, 2, 2, 8, 5, 0, 3, 4, 4, 1];

    let mut v1 = nums.to_vec();
    v1.sort();

    let v_len = v1.len();
    let median = v1[v_len / 2];

    println!("Median: {median}");
}