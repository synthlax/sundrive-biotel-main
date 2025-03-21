#[derive(Copy, Clone)]
pub enum IUPACNucleotide {
    A { letter: char, name: &'static str }, 
    C { letter: char, name: &'static str }, 
    G { letter: char, name: &'static str }, 
    T { letter: char, name: &'static str }, 
    U { letter: char, name: &'static str }, 
    R { letter: char, name: &'static str },  
    Y { letter: char, name: &'static str },  
    S { letter: char, name: &'static str }, 
    W { letter: char, name: &'static str },  
    K { letter: char, name: &'static str },  
    M { letter: char, name: &'static str },  
    B { letter: char, name: &'static str },  
    D { letter: char, name: &'static str },  
    H { letter: char, name: &'static str },  
    V { letter: char, name: &'static str },  
    N { letter: char, name: &'static str },  
}

pub enum AminoAcid {
    Alanine { codons: [[IUPACNucleotide; 3]; 4], name: &'static str },
    Arginine { codons: [IUPACNucleotide; 3], name: &'static str },
    Asparagine { codons: [IUPACNucleotide; 3], name: &'static str },
    AsparticAcid { codons: [IUPACNucleotide; 3], name: &'static str },
    Cysteine { codons: [IUPACNucleotide; 3], name: &'static str },
    Glutamine { codons: [IUPACNucleotide; 3], name: &'static str },
    GlutamicAcid { codons: [IUPACNucleotide; 3], name: &'static str },
    Glycine { codons: [IUPACNucleotide; 3], name: &'static str },
    Histidine { codons: [IUPACNucleotide; 3], name: &'static str },
    Isoleucine { codons: [IUPACNucleotide; 3], name: &'static str },
    Leucine { codons: [IUPACNucleotide; 3], name: &'static str },
    Lysine { codons: [IUPACNucleotide; 3], name: &'static str },
    Methionine { codons: [IUPACNucleotide; 3], name: &'static str },
    Phenylalanine { codons: [IUPACNucleotide; 3], name: &'static str },
    Proline { codons: [IUPACNucleotide; 3], name: &'static str },
    Serine { codons: [IUPACNucleotide; 3], name: &'static str },
    Threonine { codons: [IUPACNucleotide; 3], name: &'static str },
    Tryptophan { codons: [IUPACNucleotide; 3], name: &'static str },
    Tyrosine { codons: [IUPACNucleotide; 3], name: &'static str },
    Valine { codons: [IUPACNucleotide; 3], name: &'static str },
    Stop { codons: [IUPACNucleotide; 3], name: &'static str }
}

static A: IUPACNucleotide = IUPACNucleotide::A { letter: 'A', name: "Adenine" };
static C: IUPACNucleotide = IUPACNucleotide::C { letter: 'C', name: "Cytosine" };
static G: IUPACNucleotide = IUPACNucleotide::G { letter: 'G', name: "Guanine" };
static T: IUPACNucleotide = IUPACNucleotide::T { letter: 'T', name: "Thymine" };
static U: IUPACNucleotide = IUPACNucleotide::U { letter: 'U', name: "Uracil (RNA)" };
static R: IUPACNucleotide = IUPACNucleotide::R { letter: 'R', name: "Purine (A or G)" };
static Y: IUPACNucleotide = IUPACNucleotide::Y { letter: 'Y', name: "Pyrimidine (C or T)" };
static S: IUPACNucleotide = IUPACNucleotide::S { letter: 'S', name: "Strong (G or C)" };
static W: IUPACNucleotide = IUPACNucleotide::W { letter: 'W', name: "Weak (A or T)" };
static K: IUPACNucleotide = IUPACNucleotide::K { letter: 'K', name: "Keto (G or T)" };
static M: IUPACNucleotide = IUPACNucleotide::M { letter: 'M', name: "Amino (A or C)" };
static B: IUPACNucleotide = IUPACNucleotide::B { letter: 'B', name: "Not A (C, G, or T)" };
static D: IUPACNucleotide = IUPACNucleotide::D { letter: 'D', name: "Not C (A, G, or T)" };
static H: IUPACNucleotide = IUPACNucleotide::H { letter: 'H', name: "Not G (A, C, or T)" };
static V: IUPACNucleotide = IUPACNucleotide::V { letter: 'V', name: "Not T (A, C, or G)" };
static N: IUPACNucleotide = IUPACNucleotide::N { letter: 'N', name: "Unknown (Any nucleotide)" };


// GCA, GCC, GCG, GCT


pub const AMINO_ACIDS: [AminoAcid; 1] = [
    AminoAcid::Alanine { codons: [[G,C,A], [G,C,C], [G,C,G], [G,C,T]], name: "Alanine" },
];

impl IUPACNucleotide {
    pub const fn get_dna_complement(&original: IUPACNucleotide) -> &IUPACNucleotide {
        let char = original.letter;
        match char {
            A => IUPACNucleotide::T,
            T => IUPACNucleotide::A,
            C => IUPACNucleotide::G { letter: 'G' },
            G => IUPACNucleotide::C
        }
    }

    /// Returns a list of all IUPAC nucleotide variants
    pub const fn all() -> &'static [IUPACNucleotide] {
        &[
            IUPACNucleotide::A { letter: 'A', name: "Adenine" },
            IUPACNucleotide::C { letter: 'C', name: "Cytosine" },
            IUPACNucleotide::G { letter: 'G', name: "Guanine" },
            IUPACNucleotide::T { letter: 'T', name: "Thymine" },
            IUPACNucleotide::U { letter: 'U', name: "Uracil (RNA)" },
            IUPACNucleotide::R { letter: 'R', name: "Purine (A or G)" },
            IUPACNucleotide::Y { letter: 'Y', name: "Pyrimidine (C or T)" },
            IUPACNucleotide::S { letter: 'S', name: "Strong (G or C)" },
            IUPACNucleotide::W { letter: 'W', name: "Weak (A or T)" },
            IUPACNucleotide::K { letter: 'K', name: "Keto (G or T)" },
            IUPACNucleotide::M { letter: 'M', name: "Amino (A or C)" },
            IUPACNucleotide::B { letter: 'B', name: "Not A (C, G, or T)" },
            IUPACNucleotide::D { letter: 'D', name: "Not C (A, G, or T)" },
            IUPACNucleotide::H { letter: 'H', name: "Not G (A, C, or T)" },
            IUPACNucleotide::V { letter: 'V', name: "Not T (A, C, or G)" },
            IUPACNucleotide::N { letter: 'N', name: "Unknown (Any nucleotide)" },
        ]
    }
    // Returns the single-letter code for this nucleotide
    pub fn letter(&self) -> char {
        match self {
            IUPACNucleotide::A { .. } => 'A',
            IUPACNucleotide::C { .. } => 'C',
            IUPACNucleotide::G { .. } => 'G',
            IUPACNucleotide::T { .. } => 'T',
            IUPACNucleotide::U { .. } => 'U',
            IUPACNucleotide::R { .. } => 'R',
            IUPACNucleotide::Y { .. } => 'Y',
            IUPACNucleotide::S { .. } => 'S',
            IUPACNucleotide::W { .. } => 'W',
            IUPACNucleotide::K { .. } => 'K',
            IUPACNucleotide::M { .. } => 'M',
            IUPACNucleotide::B { .. } => 'B',
            IUPACNucleotide::D { .. } => 'D',
            IUPACNucleotide::H { .. } => 'H',
            IUPACNucleotide::V { .. } => 'V',
            IUPACNucleotide::N { .. } => 'N',
        }
    }

    // Returns the full name of the nucleotide
    pub fn full_name(&self) -> &'static str {
        match self {
            IUPACNucleotide::A { .. } => "Adenine",
            IUPACNucleotide::C { .. } => "Cytosine",
            IUPACNucleotide::G { .. } => "Guanine",
            IUPACNucleotide::T { .. } => "Thymine",
            IUPACNucleotide::U { .. } => "Uracil",
            IUPACNucleotide::R { .. } => "Purine (A or G)",
            IUPACNucleotide::Y { .. } => "Pyrimidine (C or T)",
            IUPACNucleotide::S { .. } => "Strong (G or C)",
            IUPACNucleotide::W { .. } => "Weak (A or T)",
            IUPACNucleotide::K { .. } => "Keto (G or T)",
            IUPACNucleotide::M { .. } => "Amino (A or C)",
            IUPACNucleotide::B { .. } => "Not A (C, G, or T)",
            IUPACNucleotide::D { .. } => "Not C (A, G, or T)",
            IUPACNucleotide::H { .. } => "Not G (A, C, or T)",
            IUPACNucleotide::V { .. } => "Not T (A, C, or G)",
            IUPACNucleotide::N { .. } => "Any nucleotide (A, C, G, T)",
        }
    }
    pub fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_uppercase() {
            'A' => Some(IUPACNucleotide::A { letter: 'A', name: "Adenine" }),
            'C' => Some(IUPACNucleotide::C { letter: 'C', name: "Cytosine" }),
            'G' => Some(IUPACNucleotide::G { letter: 'G', name: "Guanine" }),
            'T' => Some(IUPACNucleotide::T { letter: 'T', name: "Thymine" }),
            'U' => Some(IUPACNucleotide::U { letter: 'U', name: "Uracil" }),
            'R' => Some(IUPACNucleotide::R { letter: 'R', name: "Purine (A or G)" }),
            'Y' => Some(IUPACNucleotide::Y { letter: 'Y', name: "Pyrimidine (C or T)" }),
            'S' => Some(IUPACNucleotide::S { letter: 'S', name: "Strong (G or C)" }),
            'W' => Some(IUPACNucleotide::W { letter: 'W', name: "Weak (A or T)" }),
            'K' => Some(IUPACNucleotide::K { letter: 'K', name: "Keto (G or T)" }),
            'M' => Some(IUPACNucleotide::M { letter: 'M', name: "Amino (A or C)" }),
            'B' => Some(IUPACNucleotide::B { letter: 'B', name: "Not A (C, G, or T)" }),
            'D' => Some(IUPACNucleotide::D { letter: 'D', name: "Not C (A, G, or T)" }),
            'H' => Some(IUPACNucleotide::H { letter: 'H', name: "Not G (A, C, or T)" }),
            'V' => Some(IUPACNucleotide::V { letter: 'V', name: "Not T (A, C, or G)" }),
            'N' => Some(IUPACNucleotide::N { letter: 'N', name: "Any nucleotide (A, C, G, T)" }),
            _ => None,
        }
    }
    
    pub fn get_bases_map() -> HashMap<String, i32> {
        let mut iupac_map: HashMap<String, i32> = HashMap::new();

        // Initialize all nucleotides with a default value of 0
        for nucleotide in IUPACNucleotide::all() {
            iupac_map.insert(nucleotide.letter().to_string().clone(), 0);
        }
        iupac_map
    }

    pub fn is_valid_sequence(sequence_str: &str) -> bool {
        let mut is_valid: bool = false;
        let nucleotide_table_ref = IUPACNucleotide::get_bases_map();
        for base in sequence_str.chars() {
            if nucleotide_table_ref.contains_key(&base.to_string()) {
                is_valid = true;
            }
            else {
                is_valid = false;
                return is_valid; 
            }
        }
        is_valid
    }

    pub fn count_bases(sequence_str: &str) -> HashMap<String, i32> {
        let mut nucleotide_table = IUPACNucleotide::get_bases_map();
        for base in sequence_str.chars() {
            let base = base.to_string();
            if nucleotide_table.contains_key(&base) {
                let count = nucleotide_table.get_mut(&base).unwrap();
                *count += 1;
            }
            else {
                nucleotide_table.insert(base, 0);
            }
        }
        nucleotide_table
    }
}

impl AminoAcid {

}