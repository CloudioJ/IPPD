#  Processamento de Textos em Paralelo

O projeto visa criar um programa multithreading para processar textos em paralelo, utilizando técnicas básicas de Processamento de Linguagem Natural (PLN) em Rust e paralelizando a execução com OpenMP. O programa lê um dataset CSV contendo e-mails que podem ser classificados em até seis categorias. Para cada classificação, o código separa os e-mails em novos arquivos CSV e conta a frequência de palavras de cada um. 

## Dataset Utilizado

O dataset utilizado para dividir as mensagens de acordo com as categorias "toxic", "severe_toxic", "obscene", "threat", "insult", "identity_hate" foi encontrado na plataforma Kaggle, através do seguinte local: https://www.kaggle.com/code/jeremyarancio/toxic-comments-classification-nb-logistic/input.

## Features

- [x] Leitura e escrita em CSV;
- [x] Remoção de pontuação e stopwords;
- [x] Tokenização;
- [ ] Contagem de frequência de palavras;
- [ ] Representação de dados.

## Autores

- Claudio Luis da Silva Machado Junior
- Fernanda Cardoso Petiz