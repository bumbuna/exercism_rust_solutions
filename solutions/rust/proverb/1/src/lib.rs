pub fn build_proverb(list: &[&str]) -> String {
    let mut s = String::new();
    if list.len() >= 1 {
        for i in 0..list.len()-1 {
            s.push_str(format!("For want of a {} the {} was lost.\n", list[i], list[i+1]).as_str())
        }
        s.push_str(format!("And all for the want of a {}.", list[0]).as_str());
    }
    s
}
