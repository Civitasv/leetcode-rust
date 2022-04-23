fn main() {}
fn reverse_words(s: String) -> String {
    let mut arr: Vec<&str> = s.split(' ').collect();
    let ret: Vec<String> = arr
        .iter_mut()
        .map(|word| -> String {
            let c: String = word.chars().rev().collect();
            c
        })
        .collect();
    let a = [1, 2, 3];

    ret.join(" ").to_string()
}
