// /*
//     Commented segments are currently disabled for simplicity until future implemenations.
// */

// use std::collections::HashMap;

// use crate::error::nucleotide_error::NucleotideError;

// #[allow(unused)]
// #[derive(Copy, Clone, PartialEq)]
// pub enum IUPACNucleotide {
//     A { letter: char, name: &'static str }, 
//     C { letter: char, name: &'static str }, 
//     G { letter: char, name: &'static str }, 
//     T { letter: char, name: &'static str }, 
//     U { letter: char, name: &'static str },
//     N { letter: char, name: &'static str } 
//     // R { letter: char, name: &'static str },  
//     // Y { letter: char, name: &'static str },  
//     // S { letter: char, name: &'static str }, 
//     // W { letter: char, name: &'static str },  
//     // K { letter: char, name: &'static str },  
//     // M { letter: char, name: &'static str },  
//     // B { letter: char, name: &'static str },  
//     // D { letter: char, name: &'static str },  
//     // H { letter: char, name: &'static str },  
//     // V { letter: char, name: &'static str },  
//     // N { letter: char, name: &'static str },  
// }

// #[allow(unused)]
// pub enum AminoAcid {
//     Alanine { codons: [[IUPACNucleotide; 3]; 4], name: &'static str },
//     Arginine { codons: [IUPACNucleotide; 3], name: &'static str },
//     Asparagine { codons: [IUPACNucleotide; 3], name: &'static str },
//     AsparticAcid { codons: [IUPACNucleotide; 3], name: &'static str },
//     Cysteine { codons: [IUPACNucleotide; 3], name: &'static str },
//     Glutamine { codons: [IUPACNucleotide; 3], name: &'static str },
//     GlutamicAcid { codons: [IUPACNucleotide; 3], name: &'static str },
//     Glycine { codons: [IUPACNucleotide; 3], name: &'static str },
//     Histidine { codons: [IUPACNucleotide; 3], name: &'static str },
//     Isoleucine { codons: [IUPACNucleotide; 3], name: &'static str },
//     Leucine { codons: [IUPACNucleotide; 3], name: &'static str },
//     Lysine { codons: [IUPACNucleotide; 3], name: &'static str },
//     Methionine { codons: [IUPACNucleotide; 3], name: &'static str },
//     Phenylalanine { codons: [IUPACNucleotide; 3], name: &'static str },
//     Proline { codons: [IUPACNucleotide; 3], name: &'static str },
//     Serine { codons: [IUPACNucleotide; 3], name: &'static str },
//     Threonine { codons: [IUPACNucleotide; 3], name: &'static str },
//     Tryptophan { codons: [IUPACNucleotide; 3], name: &'static str },
//     Tyrosine { codons: [IUPACNucleotide; 3], name: &'static str },
//     Valine { codons: [IUPACNucleotide; 3], name: &'static str },
//     Stop { codons: [IUPACNucleotide; 3], name: &'static str }
// }

// const IUPAC_NUCLEOTIDE_TABLE: [IUPACNucleotide; 6] = [    
//     IUPACNucleotide::A { letter: 'A', name: "Adenine" },
//     IUPACNucleotide::T { letter: 'T', name: "Thymine" },
//     IUPACNucleotide::C { letter: 'C', name: "Cytosine" },
//     IUPACNucleotide::G { letter: 'G', name: "Guanine" },
//     IUPACNucleotide::U { letter: 'U', name: "Uracil" },
//     IUPACNucleotide::N { letter: 'N', name: "Unknown" },
// ];

// #[allow(unused)]
// impl IUPACNucleotide {
//     // Returns the full name of the nucleotide
//     pub fn full_name(&self) -> &'static str {
//         match self {
//             IUPACNucleotide::A { .. } => "Adenine",
//             IUPACNucleotide::C { .. } => "Cytosine",
//             IUPACNucleotide::G { .. } => "Guanine",
//             IUPACNucleotide::T { .. } => "Thymine",
//             IUPACNucleotide::U { .. } => "Uracil",
//             IUPACNucleotide::N { .. } => "Unkown",
//             // IUPACNucleotide::R { .. } => "Purine (A or G)",
//             // IUPACNucleotide::Y { .. } => "Pyrimidine (C or T)",
//             // IUPACNucleotide::S { .. } => "Strong (G or C)",
//             // IUPACNucleotide::W { .. } => "Weak (A or T)",
//             // IUPACNucleotide::K { .. } => "Keto (G or T)",
//             // IUPACNucleotide::M { .. } => "Amino (A or C)",
//             // IUPACNucleotide::B { .. } => "Not A (C, G, or T)",
//             // IUPACNucleotide::D { .. } => "Not C (A, G, or T)",
//             // IUPACNucleotide::H { .. } => "Not G (A, C, or T)",
//             // IUPACNucleotide::V { .. } => "Not T (A, C, or G)",
//             // IUPACNucleotide::N { .. } => "Any nucleotide (A, C, G, T)",
//         }
//     }

//     pub fn to_char(nucleotide: IUPACNucleotide) -> Option<char> {
//         match nucleotide {
//             IUPACNucleotide::A { .. } => Some('A'),
//             IUPACNucleotide::C { .. } => Some('C'),
//             IUPACNucleotide::G { .. } => Some('G'),
//             IUPACNucleotide::T { .. } => Some('T'),
//             IUPACNucleotide::U { .. } => Some('U'),
//             IUPACNucleotide::N { .. } => Some('N'),
//             _ => Some('N')
//         }
//     }

//     pub fn from_char(c: char) -> Option<Self> {
//         match c.to_ascii_uppercase() {
//             'A' => Some(IUPACNucleotide::A { letter: 'A', name: "Adenine" }),
//             'C' => Some(IUPACNucleotide::C { letter: 'C', name: "Cytosine" }),
//             'G' => Some(IUPACNucleotide::G { letter: 'G', name: "Guanine" }),
//             'T' => Some(IUPACNucleotide::T { letter: 'T', name: "Thymine" }),
//             'U' => Some(IUPACNucleotide::U { letter: 'U', name: "Uracil" }),
//             'N' => Some(IUPACNucleotide::N { letter: 'N', name: "Any nucleotide (A, C, G, T)" }),
//             // 'R' => Some(IUPACNucleotide::R { letter: 'R', name: "Purine (A or G)" }),
//             // 'Y' => Some(IUPACNucleotide::Y { letter: 'Y', name: "Pyrimidine (C or T)" }),
//             // 'S' => Some(IUPACNucleotide::S { letter: 'S', name: "Strong (G or C)" }),
//             // 'W' => Some(IUPACNucleotide::W { letter: 'W', name: "Weak (A or T)" }),
//             // 'K' => Some(IUPACNucleotide::K { letter: 'K', name: "Keto (G or T)" }),
//             // 'M' => Some(IUPACNucleotide::M { letter: 'M', name: "Amino (A or C)" }),
//             // 'B' => Some(IUPACNucleotide::B { letter: 'B', name: "Not A (C, G, or T)" }),
//             // 'D' => Some(IUPACNucleotide::D { letter: 'D', name: "Not C (A, G, or T)" }),
//             // 'H' => Some(IUPACNucleotide::H { letter: 'H', name: "Not G (A, C, or T)" }),
//             // 'V' => Some(IUPACNucleotide::V { letter: 'V', name: "Not T (A, C, or G)" }),
//             _ => None,
//         }
//     }

//     //transforms All IUPAC enums into characters
//     pub fn get_bases_map() -> HashMap<char, i32> {
//         let mut iupac_map: HashMap<char, i32> = HashMap::new();

//         // Initialize all nucleotides with a default value of 0
//         for nucleotide in IUPAC_NUCLEOTIDE_TABLE {
//             iupac_map.insert(IUPACNucleotide::to_char(nucleotide).unwrap(), 0);
//         }
//         iupac_map
//     }

//     // Evaluates nucletoide base pair type from a sequence of a specified length
//     pub fn get_nucleotide_type(sequence_str: &str) -> Result<String, NucleotideError> {
//         let mut is_dna = false;
//         let mut is_rna = false;
//         let mut is_invalid = false;
//         for base in sequence_str.chars() {
//             if !IUPAC_NUCLEOTIDE_TABLE.contains(&Self::from_char(base).unwrap()) {
//                 is_invalid;
//                 return Err(NucleotideError::UBPError)
//             }
//             match base {
//                 'U' => is_rna = true,
//                 'T' => is_dna = true,
//                 _ => continue
//             }
//         }
//         if is_dna && !is_rna {
//             Ok("DNA".to_string())
//         }
//         else if is_rna && !is_dna {
//             Ok("RNA".to_string())
//         }
//         else {
//             Err(NucleotideError::UBPError)
//         }
//     }

//     pub fn count_nucleotides(sequence_str: &str) -> Option<HashMap<char, i32>> {
//         let mut nucleotide_table = IUPACNucleotide::get_bases_map();
//         for base in sequence_str.chars() {
//             if nucleotide_table.contains_key(&base) {
//                 let count = nucleotide_table.get_mut(&base).unwrap();
//                 *count += 1;
//             }
//             else {
//                 nucleotide_table.insert(base, 0);
//             }
//         }
//         Some(nucleotide_table)
//     }
    

// }
