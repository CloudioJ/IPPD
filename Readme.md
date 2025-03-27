#  Processamento de Textos em Paralelo

O projeto visa criar um programa multithreading para processar textos em paralelo, utilizando técnicas básicas de Processamento de Linguagem Natural (PLN) em Rust e paralelizando a execução com OpenCL. O programa lê um dataset CSV contendo e-mails que podem ser classificados em até seis categorias. Para cada classificação, o código separa os e-mails em novos arquivos txt e conta a frequência de palavras de cada um utilizando a GPU. 

## Dataset Utilizado

O dataset utilizado para dividir as mensagens de acordo com as categorias "toxic", "severe_toxic", "obscene", "threat", "insult", "identity_hate" foi encontrado na plataforma Kaggle: [Toxic Comments Classification - NB-Logistic](https://www.kaggle.com/code/jeremyarancio/toxic-comments-classification-nb-logistic/input).

## Funcionalidades

- [x] **Leitura e Escrita em CSV**: Leitura de dados de um arquivo CSV de entrada e escrita de dados processados em arquivos CSV de saída;
- [x] **Remoção de Pontuação e Stopwords**: Limpeza do texto removendo pontuações e palavras comuns (stopwords);
- [x] **Tokenização**: Divisão do texto em palavras individuais (tokens);
- [x] **Cálculo de Frequência de Palavras com GPU:** Utiliza a GPU para calcular a frequência de palavras em cada categoria.
- [x] **Representação de Dados**: Exibição das 10 palavras mais frequentes de cada categoria.
- [x] **Limpeza de Arquivos Temporários:** Remove os arquivos de texto temporários após a análise.

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
        cd <diretório_do_repositório>
        ```

2.  **Execute o programa:**

    ```bash
    cargo build --release
    cargo run
    ```

## Estrutura do Projeto

    O projeto está organizado nos seguintes diretórios:

    -   `csv/`: Contém o dataset original;
    -   `src/`: Contém o arquivo main.rs;
    -   `txt/`: Contémos os arquivos de texto por categoria - crie manualmente o diretório.
    
## Autores

- Claudio Luis da Silva Machado Junior
- Fernanda Cardoso Petiz