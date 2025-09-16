pub fn reverse(input: &str) -> String {
    let mut ret: String = String::new();
    for c in input.chars() {
        ret = format!("{c}{ret}");
    }
    ret
}
