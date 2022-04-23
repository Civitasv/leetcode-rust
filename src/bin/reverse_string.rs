fn main() {}
fn reverse_string(s: &mut Vec<char>) {
    let mut lo = 0;
    let mut hi = s.len() - 1;
    while lo < hi {
        let temp = s[lo];
        s[lo] = s[hi];
        s[hi] = temp;
        lo += 1;
        hi -= 1;
    }
}
