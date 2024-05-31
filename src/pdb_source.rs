use std::fs::File;
use pdb::{FallibleIterator, ItemFinder, PDB, TypeIndex, TypeInformation};

pub struct PDBSource<'a> {
    pdb: PDB<'a, File>,
    type_information: TypeInformation<'a>,
    //type_finder: ItemFinder<'a, TypeIndex>,
}
impl<'a> PDBSource<'a> {
    pub fn new<'b :'a>(path: &str) -> pdb::Result<PDBSource<'a>> {
        let file = File::open(path)?;
        let mut pdb = PDB::open(file)?;

        let type_information = pdb.type_information()?;

        Ok(Self {
            pdb,
            type_information,
            //type_finder,
        })
    }
    pub fn pdb(&'a self) -> &PDB<File> {
        &self.pdb
    }
    pub fn type_information(&'a self) -> &TypeInformation {
        &self.type_information
    }
    pub fn type_finder(&'a self) -> pdb::Result<ItemFinder<'a, TypeIndex>> {

        let mut type_finder = self.type_information.finder();

        let mut type_iter = self.type_information.iter();
        while let Some(_) = type_iter.next()? {
            type_finder.update(&type_iter);
        }

        Ok(type_finder)
    }
}
