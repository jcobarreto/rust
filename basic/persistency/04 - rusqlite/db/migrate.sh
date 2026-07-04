#!/bin/bash

# Nome do arquivo de banco de dados
DB_FILE="rusqlite.db"

# Caminho para o arquivo SQL de criação das tabelas
SQL_FILE="script.sql"

# Verifica se o arquivo de banco de dados já existe
if [ -f "$DB_FILE" ]; then
    echo "Database '$DB_FILE' already exists."
    exit 1
fi

# Executa os comandos SQL contidos no arquivo
sqlite3 "$DB_FILE" < "$SQL_FILE"

echo "Database '$DB_FILE' created successfully."
