use std::error::Error;
use std::fs::{File, OpenOptions};
use csv::Reader;
use std::collections::{HashSet, HashMap};
use std::time::Instant;
use rayon::prelude::*;
use regex::Regex;
use std::io::{BufReader, Write};
use ocl::{Buffer, Kernel, Platform, Program, Queue, Device, Context};

/* ==============================================
    word_frequency_gpu
    ----------------------------------------------
    Entrada: um vetor de strings;
    Saída: um HashMap com "palavra: frequência".
    ==============================================  */
fn word_frequency_gpu(words: Vec<String>) -> HashMap<String, usize> {
    let platform = Platform::default();
    println!("Plataforma: {:?}", platform.name()); // Imprimir a plataforma
    let device = Device::first(platform).unwrap();
    println!("Dispositivo: {:?}", device.name()); // Imprimir o nome da GPU

    let context = Context::builder()
        .platform(platform)
        .devices(device)
        .build()
        .unwrap();
    let queue = Queue::new(&context, device, None).unwrap();

    // Criando um mapa de palavras para índices
    let mut word_to_index: HashMap<String, usize> = HashMap::new();
    let mut index_to_word: Vec<String> = Vec::new();

    for word in &words {
        if !word_to_index.contains_key(word) {
            let idx = index_to_word.len();
            word_to_index.insert(word.clone(), idx);
            index_to_word.push(word.clone());
        }
    }

    let indices: Vec<i32> = words
        .iter()
        .map(|w| *word_to_index.get(w).unwrap() as i32)
        .collect();

    let buffer = Buffer::<i32>::builder()
        .queue(queue.clone())
        .len(indices.len())
        .copy_host_slice(&indices)
        .build()
        .unwrap();

    let mut counts = vec![0; word_to_index.len()];
    let buffer_out = Buffer::<i32>::builder()
        .queue(queue.clone())
        .len(counts.len())
        .copy_host_slice(&counts)
        .build()
        .unwrap();

    let program = Program::builder()
        .src("
            __kernel void count_words(__global int* input, __global int* output, int n) {
                int id = get_global_id(0);
                if (id < n) {
                    atomic_add(&output[input[id]], 1);
                }
            }
        ")
        .devices(device)
        .build(&context)
        .unwrap();

    let kernel = Kernel::builder()
        .program(&program)
        .name("count_words")
        .queue(queue.clone())
        .global_work_size(indices.len())
        .arg(&buffer)
        .arg(&buffer_out)
        .arg(&(indices.len() as i32))
        .build()
        .unwrap();

    // Executar o kernel na GPU
    unsafe { kernel.enq().unwrap(); }
    println!("Kernel executado com sucesso.");

    buffer_out.read(&mut counts).enq().unwrap();

    let mut freq_map = HashMap::new();
    for (i, &count) in counts.iter().enumerate() {
        if count > 0 {
            freq_map.insert(index_to_word[i].clone(), count as usize);
        }
    }

    freq_map
}

/* ==============================================
    append_txt
    ----------------------------------------------
    Entrada: caminho do arquivo de uma categoria e
    uma string;
    Saída: escreve a string ao final do txt ou erro.
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
    let mut rdr = Reader::from_reader(BufReader::new(file));
    
    let re = Regex::new(r"\b\w+\b").unwrap();
    
    let words: Vec<String> = rdr.records()
        .par_bridge()
        .filter_map(|result| {
            if let Ok(record) = result {
                let text = record.iter().collect::<Vec<&str>>().join(" ");
                Some(re.find_iter(&text)
                    .map(|m| m.as_str().to_lowercase())
                    .collect::<Vec<String>>())
            } else {
                None
            }
        })
        .flatten()
        .collect();
    
    word_frequency_gpu(words)
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
