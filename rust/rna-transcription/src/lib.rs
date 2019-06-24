#[derive(Debug, PartialEq)]
pub struct DNA(String);

#[derive(Debug, PartialEq)]
pub struct RNA(String);

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        match dna.chars().position(|x| !vec!['A','T','G', 'C'].contains(&x)){
            Some(pos) => Err(pos),
            None => {
                Ok(DNA(dna.to_string()))
            }
        }
    }

    pub fn into_rna(self) -> RNA {
        RNA(self.0.chars().map(|x|{
            match x{
                'A' => 'U',
                'T' => 'A',
                'G' => 'C',
                'C' => 'G',
                _ => panic!("This can't happen!")
            }
        }).collect())
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        match rna.chars().position(|x| !vec!['A','U','G', 'C'].contains(&x)){
            Some(pos) => Err(pos),
            None => {
                Ok(RNA(rna.to_string()))
            }
        }
    }
}
