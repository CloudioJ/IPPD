use std::error::Error;
use std::fs::File;
use std::path::Path;
use csv::Reader;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "src/arquivo.csv";

    let file = File::open(filename)?;
    let mut rdr = Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        if let Some(field) = record.get(2) {
            println!("{}", field);
        }
    }

    println!("{}", filename);
    
    Ok(())
}
