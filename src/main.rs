use std::error::Error;
use std::fs::{File, OpenOptions};
use csv::{Reader};
use std::collections::{HashSet, HashMap};
use std::time::Instant;
use rayon::prelude::*;
use regex::Regex;
use std::io::{BufReader, Write};
use std::sync::Mutex;

/* ==============================================
    most_frequent_strings
    ----------------------------------------------
    Entrada: um vetor de strings e um valor máximo;
    Saída: um vetor de tuplas (palavra, frequência).
    ==============================================  */
fn most_frequent_strings(strings: Vec<&str>, limit: usize) -> Vec<(&str, usize)> {
    let mut frequency_map: HashMap<&str, usize> = HashMap::new();
    for s in strings.iter() {
        *frequency_map.entry(s).or_insert(0) += 1;
    }
    let mut word_counts: Vec<(&str, usize)> = frequency_map.into_iter().collect();
    word_counts.sort_by(|a, b| b.1.cmp(&a.1));
    word_counts.into_iter().take(limit).collect()
}

/* ==============================================
    append_txt
    ----------------------------------------------
    Entrada: caminho do TXT da categoria, string
    com as palavras a serem escritas nele;
    Saída: Os dados são escritos ou retorna erro.
    ==============================================  */
fn append_txt(file_path: &str, data: &str) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;
    writeln!(file, "{}", data)?; // Write each line separately
    Ok(())
}

/* ==============================================
    split_csv
    ----------------------------------------------
    Entrada: linha do CSV original, texto da linha;
    Saída: os dados são distribuídos nos TXTs de
    cada categoria ou erro.
    ==============================================  */
fn split_csv(record: csv::StringRecord, text: String) -> Result<(), Box<dyn Error>> {
    let files = [
        "txt/toxic.txt", "txt/severe_toxic.txt", "txt/obscene.txt",
        "txt/threat.txt", "txt/insult.txt", "txt/identity_hate.txt"
    ];

    for (i, file) in files.iter().enumerate() {
        if let Ok(val) = record[i + 2].parse::<u8>() {
            if val == 1 {
                append_txt(file, &text)?;
            }
        }
    }
    Ok(())
}

/*  ==============================================  
    word_frequency_from_csv
    ----------------------------------------------
    Entrada: caminho do csv de uma categoria;
    Saída: HashMap com "palavra: frequência".
    ==============================================  */
fn word_frequency_from_txt(file_path: &str) -> HashMap<String, usize> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let word_count = Mutex::new(HashMap::new());
    let re = Regex::new(r"\b\w+\b").unwrap();

    use std::io::BufRead;
    reader.lines().par_bridge().for_each(|line_result| {
        if let Ok(line) = line_result {
            let words = re.find_iter(&line).map(|m| m.as_str().to_lowercase());

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

/* ==============================================
    main
    ----------------------------------------------
    Lê CSV original e filtra de acordo com as
    técnicas básicas de PLN. Mostra o tempo total
    de execução.
    ==============================================  */
fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    println!("Iniciando a execução do programa...");
    let filename = "csv/dataset.csv";
    let files = [
        "txt/toxic.txt", "txt/severe_toxic.txt", "txt/obscene.txt",
        "txt/threat.txt", "txt/insult.txt", "txt/identity_hate.txt"
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

    let _word_list: Vec<String> = rdr.records().par_bridge().filter_map(|result| {
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

    for file in files.iter() {
        let word_freq = word_frequency_from_txt(file);
        println!("Frequência em {}:", file);

        let mut sorted_freq: Vec<(&String, &usize)> = word_freq.iter().collect();
        sorted_freq.sort_by(|a, b| b.1.cmp(a.1));

        for (word, count) in sorted_freq.iter().take(10) {
            println!("{}: {}", word, count);
        }
        println!();
    }
    
    // files.par_iter().for_each(|file| {
    //     if let Err(err) = std::fs::remove_file(file) {
    //         eprintln!("Erro ao excluir {}: {}", file, err);
    //     } else {
    //         println!("Arquivo {} excluído.", file);
    //     }
    // });

    println!("Tempo de execução: {:?}", start.elapsed());

    Ok(())
}
