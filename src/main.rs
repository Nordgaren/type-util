pub mod type_info;
mod pdb_source;

use pdb::{FallibleIterator, TypeData, TypeIndex};
use crate::pdb_source::PDBSource;

fn main() -> pdb::Result<()> {
    let pdb = PDBSource::new(r"T:\source\CPP\Erd-Tools-CPP\x64\Release\ErdTools.pdb")?;

    {
        let type_finder = pdb.type_finder();

        let t = type_finder.find(TypeIndex(0x12e2))?.parse()?;
        println!("{t:#?}");

        match t {
            TypeData::FieldList(f) => {
                for field in f.fields {
                    match field {
                        TypeData::Member(m) => {
                            let f_t = type_finder.find(m.field_type)?.parse()?;
                            println!("{} {f_t:#?}", m.name);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        let symbol_table = pdb.pdb().global_symbols()?;
        let mut symbols = symbol_table.iter();
        while let Some(symbol) = symbols.next()? {
            match symbol.parse()? {
                pdb::SymbolData::UserDefinedType(udt) => {
                    let t = type_finder.find(udt.type_index)?.parse()?;
                    println!("{} {t:?}", udt.name.to_string());
                }
                _ => {}
            }
        }
    }

    Ok(())
}
