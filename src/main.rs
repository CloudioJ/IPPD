use std::error::Error;
use std::fs::{File, OpenOptions};
use csv::{Reader, WriterBuilder};
use std::collections::{HashSet, HashMap};
use std::time::Instant;
use rayon::prelude::*;
use regex::Regex;
use std::io::BufReader;
use std::sync::Mutex;

fn most_frequent_strings(strings: Vec<&str>, limit: usize) -> Vec<(&str, usize)> {
    let mut frequency_map: HashMap<&str, usize> = HashMap::new();
    for s in strings.iter() {
        *frequency_map.entry(s).or_insert(0) += 1;
    }
    let mut word_counts: Vec<(&str, usize)> = frequency_map.into_iter().collect();
    word_counts.sort_by(|a, b| b.1.cmp(&a.1));
    word_counts.into_iter().take(limit).collect()
}

fn append_csv(file_path: &str, data: &str) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;
    let mut wrt = WriterBuilder::new().has_headers(false).from_writer(file);
    wrt.write_record(data.split_whitespace())?;
    wrt.flush()?; // Ensure all data is written to the file
    Ok(())
}

fn split_csv(record: csv::StringRecord, text: String) -> Result<(), Box<dyn Error>> {
    let files = [
        "csv/toxic.csv", "csv/severe_toxic.csv", "csv/obscene.csv", 
        "csv/threat.csv", "csv/insult.csv", "csv/identity_hate.csv"
    ];
    
    for (i, file) in files.iter().enumerate() {
        if let Ok(val) = record[i + 2].parse::<u8>() {
            if val == 1 {
                append_csv(file, &text)?;
            }
        }
    }
    Ok(())
}

fn word_frequency_from_csv(file_path: &str) -> HashMap<String, usize> {
    let file = File::open(file_path).expect("Failed to open file");
    let mut rdr = Reader::from_reader(BufReader::new(file));

    let word_count = Mutex::new(HashMap::new());
    let re = Regex::new(r"\b\w+\b").unwrap();

    rdr.records()
        .par_bridge()
        .for_each(|result| {
            if let Ok(record) = result {
                let text = record.iter().collect::<Vec<&str>>().join(" ");
                let words = re.find_iter(&text).map(|m| m.as_str().to_lowercase());

                let mut local_count = HashMap::new();
                for word in words {
                    *local_count.entry(word).or_insert(0) += 1;
                }

                let mut global_count = word_count.lock().unwrap();
                for (word, count) in local_count {
                    *global_count.entry(word).or_insert(0) += count;
                }
            }
        });

    word_count.into_inner().unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    println!("Iniciando a execução do programa...");
    let filename = "csv/dataset.csv";
    let files = [
        "csv/toxic.csv", "csv/severe_toxic.csv", "csv/obscene.csv", 
        "csv/threat.csv", "csv/insult.csv", "csv/identity_hate.csv"
    ];
    
    let file = File::open(filename)?;
    let mut rdr = Reader::from_reader(file);
    
    let stopwords: HashSet<&str> = [
        "i", "me", "my", "myself", "we", "our", "ours", "ourselves", "you", "your", "yours", 
        "yourself", "yourselves", "he", "him", "his", "himself", "she", "her", "hers", "herself", 
        "it", "its", "itself", "they", "them", "their", "theirs", "themselves", "what", "which", 
        "who", "whom", "these", "those", "am", "is", "are", "was", "were", "be", "been", "being",
        "have", "has", "had", "having", "do", "does", "did", "doing", "a", "an", "the", "and", 
        "but", "if", "or", "because", "as", "until", "while", "of", "at", "by", "for", "with", 
        "about", "against", "between", "into", "through", "during", "before", "after", "above", 
        "below", "to", "from", "up", "down", "in", "out", "on", "off", "over", "under", "again", 
        "further", "then", "once", "here", "there", "when", "where", "why", "how", "all", "any", 
        "both", "each", "few", "more", "most", "other", "some", "such", "no", "nor", "not", "only",
        "own", "same", "so", "than", "too", "very", "s", "t", "can", "will", "just", "don", "should",
        "now", "d", "ll", "m", "o", "re", "ve", "y", "ain", "aren", "couldn", "didn", "doesn", "hadn", 
        "hasn", "haven", "isn", "ma", "mightn", "mustn", "needn", "shan", "shouldn", "wasn", "weren", 
        "won", "wouldn", "the", "is", "in", "and", "to", "of", "a", "with", "for", "on", "this", "that", 
        "it", "as", "at", "by", "be", "are", "was", "were", "from", "dont", "have", "has", "had", "do", "thats",
    ].iter().cloned().collect();
    
    let word_list: Vec<String> = rdr.records().par_bridge().filter_map(|result| {
        if let Ok(record) = result {
            if let Some(field) = record.get(1) {
                let field = field.replace(&['(', ')', ',', '"', '.', ';', ':', '\'', '?', '!'][..], "");
                let sentences: Vec<String> = field.split_whitespace()
                    .map(|x| x.to_lowercase())
                    .filter(|word| word.len() > 3 && !stopwords.contains(word.as_str()))
                    .collect();
                
                let _ = split_csv(record.clone(), sentences.join(" "));
                return Some(sentences);
            }
        }
        None
    }).flatten().collect();
    
    // let most_frequent_words = most_frequent_strings(
    //     word_list.iter().map(|x| x.as_str()).collect(),
    //     30
    // );
    
    // for (word, count) in most_frequent_words {
    //     println!("{:?}: {:?}", word, count);
    // }
    

    for file in files.iter() {
        let word_freq = word_frequency_from_csv(file);
        println!("Frequência em {}:", file);
        
        // Convert to vector and sort by frequency in descending order
        let mut sorted_freq: Vec<(&String, &usize)> = word_freq.iter().collect();
        sorted_freq.sort_by(|a, b| b.1.cmp(a.1)); // Sort by count (descending)
    
        // Print top 10 words
        for (word, count) in sorted_freq.iter().take(10) {
            println!("{}: {}", word, count);
        }
        println!();
    }
    
    files.par_iter().for_each(|file| {
        if let Err(err) = std::fs::remove_file(file) {
            eprintln!("Erro ao excluir {}: {}", file, err);
        } else {
            println!("Arquivo {} excluído.", file);
        }
    });

    println!("Tempo de execução: {:?}", start.elapsed());

    Ok(())
}
