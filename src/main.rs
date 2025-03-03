use std::error::Error;
use std::fs::File;
use csv::Reader;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "csv/dataset.csv";

    let file = File::open(filename)?;
    let mut rdr = Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        if let Some(field) = record.get(1) {
            println!("{}", field.to_lowercase());
        }
    }

    println!("{}", filename);
    
    Ok(())
}
