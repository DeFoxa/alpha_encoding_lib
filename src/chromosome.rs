use crate::gene::Gene;

#[derive(Debug)]
pub struct Chromosome {
    genes: Vec<Gene>,
}

impl Chromosome {
    fn new(genes: Vec<Gene>) -> Self {
        Chromosome { genes }
    }
}
