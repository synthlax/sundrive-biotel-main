#[allow(unused)]
pub enum NucleotideError {
    IUPACCodeNotFound,
    UBPError,
}

#[allow(unused)]
impl NucleotideError {
    pub fn to_string(&self) -> String {
        match self {
            NucleotideError::IUPACCodeNotFound => "Nucleotide IUPAC codes not found. Please follow IUPAC convention for nucleotides: https://www.insdc.org/submitting-standards/feature-table/".to_string(),
            NucleotideError::UBPError => "Unnatural Base pair found. Uracil and Thymine found in the same sequence.".to_string()
        }
    }

}


impl ToString for NucleotideError {
    fn to_string(&self) -> String {
        self.to_string().to_owned()
    }
}
