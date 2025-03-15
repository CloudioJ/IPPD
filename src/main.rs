use std::error::Error;
use std::fs::File;
use csv::Reader;
use std::collections::HashSet;
use std::collections::HashMap;

fn most_frequent_strings(strings: Vec<&str>, limit: usize) -> Vec<(&str, usize)> {
    let mut frequency_map: HashMap<&str, usize> = HashMap::new();
    
    // Count the occurrences of each string
    for s in strings.iter() {
        *frequency_map.entry(s).or_insert(0) += 1;
    }
    
    // Convert the HashMap into a Vec to sort it
    let mut word_counts: Vec<(&str, usize)> = frequency_map.into_iter().collect();
    
    // Sort by count in descending order
    word_counts.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Return the top `limit` entries
    word_counts.into_iter().take(limit).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename: &str = "csv/dataset.csv";

    let file: File = File::open(filename)?;
    let mut rdr: Reader<File> = Reader::from_reader(file);

    let stopwords: HashSet<&str> = [
        "the", "is", "in", "and", "to", "of", "a", "with", "for", "on", "this", 
        "that", "it", "as", "at", "by", "be", "are", "was", "were", "from"
    ].iter().cloned().collect();

    let mut word_list: Vec<String> = Vec::new();

    for result in rdr.records() {
        let record: csv::StringRecord = result?;
        if let Some(field) = record.get(1) {
            let field: String = field.replace(&['(', ')', ',', '\"', '.', ';', ':', '\'', '?', '!'][..], "");
            let sentences: Vec<String> = field.split_whitespace()
                .map(|x: &str| x.to_lowercase()) // Convert to lowercase
                .filter(|word: &String| word.len() > 3 && !stopwords.contains(word.as_str())) // Remove stopwords
                .collect();

            for word in sentences {
                word_list.push(word);
                // println!("{:?}", word);
            }
        }	
    }

    let most_frequent_words: Vec<(&str, usize)> = most_frequent_strings(
        word_list.iter().map(|x| x.as_str()).collect(),
        30 // Get top 30 words
    );

    for (word, count) in most_frequent_words {
        println!("{:?}: {:?}", word, count);
    }
    
    Ok(())
}
