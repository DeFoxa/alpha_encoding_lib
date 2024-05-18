use crate::gene::Gene;

#[derive(Debug)]
pub struct Chromosome {
    genes: Vec<Gene>,
}

impl Chromosome {
    fn new() -> Self {
        Chromosome { genes: Vec::new() }
    }

    fn new_from_vec(genes: Vec<Gene>) -> Self {
        Chromosome { genes }
    }

    fn push(&mut self, gene: Gene) {
        self.genes.push(gene);
    }

    fn append_vec(&mut self, genes: Vec<Gene>) {
        todo!();
    }
}
