use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use csv::Reader;
use csv::StringRecord;
use csv::Writer;
use csv::WriterBuilder;
use std::collections::HashSet;
use std::collections::HashMap;
use csv::ReaderBuilder;
use serde::Deserialize;

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


/* Função para criar e escrever em um arquivo CSV. */

    fn append_csv(file_path:&str, data: &str) -> Result<(), Box<dyn Error>> {
        let file = OpenOptions::new()
            .append(true)
            .open(file_path)?;

        let mut wrt = WriterBuilder::new().has_headers(false).from_writer(file);

        wrt.write_record(data.split_whitespace())?;
        wrt.flush();

        Ok(())
    }

/*  Função para dividir as mensagens em arquivos CSV de acordo 
    com as categrorias do cabeçalho do dataset de entrada. */

    fn split_csv(input_record: csv::StringRecord, input_text: String) -> Result<(), Box<dyn Error>> {
        
        // Processa a string senteces
        
        let record = input_record;
            
        let text = input_text;
        let toxic: u8 = record[2].parse()?;
        let severe_toxic: u8 = record[3].parse()?;
        let obscene: u8 = record[4].parse()?;
        let threat: u8 = record[5].parse()?;
        let insult: u8 = record[6].parse()?;
        let identity_hate: u8 = record[7].parse()?;
    
        // Usando match para decidir em qual arquivo a mensagem será colocada
        match (toxic, severe_toxic, obscene, threat, insult, identity_hate) {
            (1, _, _, _, _, _) => append_csv("csv/toxic.csv", &text)?,
            (_, 1, _, _, _, _) => append_csv("csv/severe_toxic.csv", &text)?,
            (_, _, 1, _, _, _) => append_csv("csv/obscene.csv", &text)?,
            (_, _, _, 1, _, _) => append_csv("csv/threat.csv", &text)?,
            (_, _, _, _, 1, _) => append_csv("csv/insult.csv", &text)?,
            (_, _, _, _, _, 1) => append_csv("csv/identity_hate.csv", &text)?,
            _ => {} // Se não se encaixar em nenhuma categoria, não faz nada
        }
    
        println!("Mensagens separadas com sucesso em arquivos por categoria.");
        Ok(())
    }

    fn frequency_counter() -> Result<(), Box<dyn Error>> {
        let file: File = File::open("csv/toxic.csv")?;
        let mut rdr: Reader<File> = Reader::from_reader(file);
    
        for result in rdr.records() {
            // Each result is a Result<StringRecord>, so we need to handle it
            let record = result?;
    
            // Now we can iterate through the fields of the record
            for field in &record {
                println!("{}", field);
            }
        }
    
        Ok(())
    }

fn main() -> Result<(), Box<dyn Error>> {
    let filename: &str = "csv/dataset.csv";

    let file: File = File::open(filename)?;
    let mut rdr: Reader<File> = Reader::from_reader(file);

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
        "it", "as", "at", "by", "be", "are", "was", "were", "from"
    ].iter().cloned().collect();

    // let mut word_list: Vec<String> = Vec::new();

    // for result in rdr.records() {
    //     let record: csv::StringRecord = result?;
    //     if let Some(field) = record.get(1) {
    //         let field: String = field.replace(&['(', ')', ',', '\"', '.', ';', ':', '\'', '?', '!'][..], "");
    //         let sentences: Vec<String> = field.split_whitespace()
    //             .map(|x: &str| x.to_lowercase()) // Convert to lowercase
    //             .filter(|word: &String| word.len() > 3 && !stopwords.contains(word.as_str())) // Remove stopwords
    //             .collect();

    //         split_csv(record, sentences.join(" "))?;

    //         for word in sentences {
    //             word_list.push(word);
    //             // println!("{:?}", word);
    //         }
    //     }	
    // }

    // let most_frequent_words: Vec<(&str, usize)> = most_frequent_strings(
    //     word_list.iter().map(|x| x.as_str()).collect(),
    //     30 // Get top 30 words
    // );

    // for (word, count) in most_frequent_words {
    //     println!("{:?}: {:?}", word, count);
    // }
    
    let _ = frequency_counter();

    Ok(())
}
