use std::collections::HashMap;
use std::str;

pub struct CodonsInfo<'a> {
    map: HashMap<&'a str, &'a str>
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        match self.map.get(codon) {
            Some(protein) => Some(*protein),
            None => None,
        }
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let vec = rna.as_bytes()
            .chunks(3)
            .map(str::from_utf8)
            .map(|codon| codon.unwrap())
            .map(|codon| self.name_for(codon))
            .take_while(|codon| !codon.is_none())
            .map(|codon| codon.unwrap())
            .take_while(|codon| *codon != "stop codon")
            .collect::<Vec<&'a str>>();
        if vec.len() < 1 {
            None
        } else {
            Some(vec)
        }
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let mut map = HashMap::new();
    for pair in pairs {
        map.insert(pair.0, pair.1);
    }
    CodonsInfo { map }
}
