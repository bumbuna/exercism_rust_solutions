use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut result = 0_usize;
    for c in dna.chars() {
        if !"ATGC".contains(c) {
            return Err(c);
        } else if !"ATCG".contains(nucleotide) {
            return Err(nucleotide);
        } else if c == nucleotide {
            result += 1;
        }
    }
    Ok(result)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut h = HashMap::new();
    for c in "ATGC".chars() {
        h.insert(c, count(c, dna)?);
    }
    Ok(h)
}
