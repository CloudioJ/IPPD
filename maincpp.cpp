#include <iostream>
#include <fstream>
#include <sstream>
#include <unordered_set>
#include <vector>
#include <regex>
#include <algorithm>

using namespace std;

// Lista de stopwords (pode ser expandida)
unordered_set<string> stopwords = {
    "a", "about", "above", "after", "again", "against", "all", "am", "an", "and",
    "any", "are", "aren't", "as", "at", "be", "because", "been", "before", "being",
    "below", "between", "both", "but", "by", "can't", "cannot", "could", "couldn't",
    "did", "didn't", "do", "does", "doesn't", "doing", "don't", "down", "during",
    "each", "few", "for", "from", "further", "had", "hadn't", "has", "hasn't",
    "have", "haven't", "having", "he", "he'd", "he'll", "he's", "her", "here",
    "here's", "hers", "herself", "him", "himself", "his", "how", "how's", "i",
    "i'd", "i'll", "i'm", "i've", "if", "in", "into", "is", "isn't", "it", "it's",
    "its", "itself", "let's", "me", "more", "most", "mustn't", "my", "myself", "no",
    "nor", "not", "of", "off", "on", "once", "only", "or", "other", "ought", "our",
    "ours", "ourselves", "out", "over", "own", "same", "shan't", "she", "she'd",
    "she'll", "she's", "should", "shouldn't", "so", "some", "such", "than", "that",
    "that's", "the", "their", "theirs", "them", "themselves", "then", "there",
    "there's", "these", "they", "they'd", "they'll", "they're", "they've", "this",
    "those", "through", "to", "too", "under", "until", "up", "very", "was", "wasn't",
    "we", "we'd", "we'll", "we're", "we've", "were", "weren't", "what", "what's",
    "when", "when's", "where", "where's", "which", "while", "who", "who's", "whom",
    "why", "why's", "with", "won't", "would", "wouldn't", "you", "you'd", "you'll",
    "you're", "you've", "your", "yours", "yourself", "yourselves"
};

// Função para remover pontuação usando regex
string removePunctuation(const string& text) {
    return regex_replace(text, regex(R"([^\w\s])"), "");
}

// Tokenização simples (divide palavras por espaço)
vector<string> tokenize(const string& text) {
    vector<string> tokens;
    stringstream ss(text);
    string word;
    
    while (ss >> word) {
        // Converte para minúsculas
        transform(word.begin(), word.end(), word.begin(), ::tolower);
        tokens.push_back(word);
    }
    return tokens;
}

// Remove stopwords da lista de tokens
vector<string> removeStopWords(const vector<string>& tokens) {
    vector<string> filteredTokens;
    
    for (const string& word : tokens) {
        if (stopwords.find(word) == stopwords.end()) { // Se não for stopword, mantém
            filteredTokens.push_back(word);
        }
    }
    return filteredTokens;
}

int main() {
    ifstream file("csv\\dataset.csv");
    if (!file) {
        cerr << "Erro ao abrir o arquivo!" << endl;
        return 1;
    }

    string line, text;
    while (getline(file, line)) {
        text += " " + line;
    }
    file.close();

    // Passo 1: Remover pontuação
    string cleanText = removePunctuation(text);

    // Passo 2: Tokenizar
    vector<string> tokens = tokenize(cleanText);

    // Passo 3: Remover stopwords
    vector<string> filteredTokens = removeStopWords(tokens);

    // Exibir resultado
    cout << "Palavras após processamento:\n";
    for (const string& word : filteredTokens) {
        cout << word << " ";
    }

    return 0;
}