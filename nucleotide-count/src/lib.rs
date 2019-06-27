use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let nucleotides: HashSet<char> = HashSet::from_iter(vec!['A', 'T', 'G', 'C']
        .iter()
        .cloned()
    );
    if dna.chars().all(|x| nucleotides.contains(&x)){
        if nucleotides.contains(&nucleotide) {
            Ok(dna.chars().filter(|x| *x == nucleotide).count())
        } else {
            Result::Err(nucleotide)
        }
    }else{
        Err(
            dna
                .chars()
                .find(|x| !nucleotides.contains(x))
                .unwrap()
        )
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let nucleotides: HashSet<char> = HashSet::from_iter(vec!['A', 'T', 'G', 'C']
        .iter()
        .cloned()
    );
    if dna.chars().all(|x| nucleotides.contains(&x)) {
        Ok(
            dna
                .chars()
                .fold(HashMap::from_iter(vec![('A', 0), ('T', 0), ('G', 0), ('C', 0)]),
                      |mut a, c| {
                          let count = a.entry(c).or_insert(0);
                          *count += 1;
                          a
                      })
        )
    } else {
        Err(
            dna
                .chars()
                .find(|x| !nucleotides.contains(x))
                .unwrap()
        )
    }
}
