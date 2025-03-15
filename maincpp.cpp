#include <iostream>
#include <fstream>
#include <sstream>
#include <unordered_set>
#include <vector>
#include <regex>
#include <algorithm>

// Lista de stopwords (pode ser expandida)
const std::unordered_set<std::string> stopwords = {"a", "o", "e", "de", "do", "da", "em", "um", "para", "com", "não", "é", "que", "se", "por", "as", "os", "como", "mas", "ou"};

// Função para remover pontuação usando regex
std::string removePunctuation(const std::string& text) {
    return std::regex_replace(text, std::regex(R"([^\w\s])"), "");
}

// Tokenização simples (divide palavras por espaço)
std::vector<std::string> tokenize(const std::string& text) {
    std::vector<std::string> tokens;
    std::stringstream ss(text);
    std::string word;
    
    while (ss >> word) {
        // Converte para minúsculas
        std::transform(word.begin(), word.end(), word.begin(), ::tolower);
        tokens.push_back(word);
    }
    return tokens;
}

// Remove stopwords da lista de tokens
std::vector<std::string> removeStopWords(const std::vector<std::string>& tokens) {
    std::vector<std::string> filteredTokens;
    
    for (const std::string& word : tokens) {
        if (stopwords.find(word) == stopwords.end()) { // Se não for stopword, mantém
            filteredTokens.push_back(word);
        }
    }
    return filteredTokens;
}

int main() {
    std::ifstream file("F:/Code/IPPD_PLN/IPPD/csv/dataset.csv");
    if (!file) {
        std::cerr << "Erro ao abrir o arquivo!" << std::endl;
        return 1;
    }

    std::string line, text;
    while (std::getline(file, line)) {
        text += " " + line;
    }
    file.close();

    // Passo 1: Remover pontuação
    std::string cleanText = removePunctuation(text);

    // Passo 2: Tokenizar
    std::vector<std::string> tokens = tokenize(cleanText);

    // Passo 3: Remover stopwords
    std::vector<std::string> filteredTokens = removeStopWords(tokens);

    // Exibir resultado
    std::cout << "Palavras após processamento:\n";
    for (const std::string& word : filteredTokens) {
        std::cout << word << " ";
    }

    return 0;
}