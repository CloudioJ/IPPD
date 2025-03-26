#  Processamento de Textos em Paralelo

O projeto visa criar um programa multithreading para processar textos em paralelo, utilizando técnicas básicas de Processamento de Linguagem Natural (PLN) em Rust e paralelizando a execução com OpenMP. O programa lê um dataset CSV contendo e-mails que podem ser classificados em até seis categorias. Para cada classificação, o código separa os e-mails em novos arquivos CSV e conta a frequência de palavras de cada um. 

## Dataset Utilizado

O dataset utilizado para dividir as mensagens de acordo com as categorias "toxic", "severe_toxic", "obscene", "threat", "insult", "identity_hate" foi encontrado na plataforma Kaggle: [Toxic Comments Classification - NB-Logistic](https://www.kaggle.com/code/jeremyarancio/toxic-comments-classification-nb-logistic/input).

## Funcionalidades

- [x] **Leitura e Escrita em CSV**: Leitura de dados de um arquivo CSV de entrada e escrita de dados processados em arquivos CSV de saída;
- [x] **Remoção de Pontuação e Stopwords**: Limpeza do texto removendo pontuações e palavras comuns (stopwords);
- [x] **Tokenização**: Divisão do texto em palavras individuais (tokens);
- [ ] **Contagem de Frequência de Palavras**: Cálculo da frequência das palavras mais comuns em cada categoria;
- [ ] **Representação de Dados**: Exibição das palavras mais frequentes e suas contagens.

## Dependências

    * Tenha certeza de ter rustc e cargo instalado. É possível verificar desta forma:

        ```bash
        rustc --version
        cargo --version
        ```
    
    Se não tiver:

        ```bash
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        ```

        ```bash
        sudo apt install rustc
        ```

## Como Executar

1.  **Clone o repositório:**

    * Se você tiver o Git instalado, clone o repositório para o seu computador usando o seguinte comando:

        ```bash
        git clone https://github.com/CloudioJ/IPPD
        ```

2.  **Execute o programa:**

    ```bash
    cargo run
    ```

## Estrutura do Projeto

    O projeto está organizado nos seguintes diretórios:

    -   `csv/`: Contém o dataset original e os gerados pelo código.
    -   `src/`: Contém o arquivo main.rs.
    
## Autores

- Claudio Luis da Silva Machado Junior
- Fernanda Cardoso Petiz