use std::error::Error;
use std::fs::File;
use csv::Reader;
fn main() -> Result<(), Box<dyn Error>> {
    let filename: &str = "csv/dataset.csv";
    let lang: &str = "en";

    let file: File = File::open(filename)?;
    let mut rdr: Reader<File> = Reader::from_reader(file);

    for result in rdr.records() {
        let record: csv::StringRecord = result?;
        if let Some(field) = record.get(1) {
            let sentences: Vec<String> = field.split(" ").map(|x: &str| x.to_string()).collect();
            for word in sentences {
                println!("{:?}", word);
            }	
        }
    }

    println!("{}", filename);
    
    Ok(())
}
