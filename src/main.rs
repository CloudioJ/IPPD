use std::error::Error;
use std::fs::File;
use csv::Reader;
use csv::Writer;
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


/*  Função para dividir as mensagens em arquvos CSV de acordo 
    com as categrorias do cabeçalho do arquivo de entrada. */

    fn split_csv(input_file: &str) -> Result<(), Box<dyn Error>> {
        // Abre csv completo
        let file = File::open(input_file)?;
        let mut rdr = Reader::from_reader(file);
    
        // Cria writers para cada categoria
        let mut wrt_toxic = Writer::from_path("csv/toxic.csv")?;
        let mut wrt_severe_toxic = Writer::from_path("csv/severe_toxic.csv")?;
        let mut wrt_obscene = Writer::from_path("csv/obscene.csv")?;
        let mut wrt_threat = Writer::from_path("csv/threat.csv")?;
        let mut wrt_insult = Writer::from_path("csv/insult.csv")?;
        let mut wrt_identity_hate = Writer::from_path("csv/identity_hate.csv")?;
    
        // Processa o CSV de entrada
        for result in rdr.records() {
            let record = result?;
            
            let text = &record[1];
            let toxic: u8 = record[2].parse()?;
            let severe_toxic: u8 = record[3].parse()?;
            let obscene: u8 = record[4].parse()?;
            let threat: u8 = record[5].parse()?;
            let insult: u8 = record[6].parse()?;
            let identity_hate: u8 = record[7].parse()?;
    
            // Usando match para decidir em qual arquivo a mensagem será colocada
            match (toxic, severe_toxic, obscene, threat, insult, identity_hate) {
                (1, _, _, _, _, _) => wrt_toxic.write_record(&[text])?,
                (_, 1, _, _, _, _) => wrt_severe_toxic.write_record(&[text])?,
                (_, _, 1, _, _, _) => wrt_obscene.write_record(&[text])?,
                (_, _, _, 1, _, _) => wrt_threat.write_record(&[text])?,
                (_, _, _, _, 1, _) => wrt_insult.write_record(&[text])?,
                (_, _, _, _, _, 1) => wrt_identity_hate.write_record(&[text])?,
                _ => {} // Se não se encaixar em nenhuma categoria, não faz nada
            }
        }
    
        // Finalizando a gravação
        wrt_toxic.flush()?;
        wrt_severe_toxic.flush()?;
        wrt_obscene.flush()?;
        wrt_threat.flush()?;
        wrt_insult.flush()?;
        wrt_identity_hate.flush()?;
    
        println!("Mensagens separadas com sucesso em arquivos por categoria.");
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
    
    split_csv(filename)?;

    Ok(())
}
