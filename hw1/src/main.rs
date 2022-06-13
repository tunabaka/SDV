fn main() {
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10];

    println!(
        "An array is sublist of another? {}",
        is_sublist(&org_arr, &sub_arr)
    );
}

fn is_sublist(a: &[i32], b: &[i32]) -> bool {
    let len_a = a.len();
    let len_b = b.len();
    let mut result = false;
    if len_a >= len_b {
        for i in 0..(len_a - len_b + 1) {
            if b == &a[i..(i + len_b)] {
                result = true;
            }
        }
    } else {
        for i in 0..(len_b - len_a + 1) {
            if a == &b[i..(i + len_a)] {
                result = true;
            }
        }
    }
    result
}
