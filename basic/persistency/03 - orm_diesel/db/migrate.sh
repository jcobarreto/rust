#!/bin/bash

DB_USER="root"
DB_PASS=""
DB_HOST="localhost"
DB_NAME="clients_rust_db"

SQL_FILE="restore.sql"

echo "Initiating database restore..."

mysql -u $DB_USER -p$DB_PASS -h $DB_HOST < $SQL_FILE

if [ $? -eq 0 ]; then
    echo "Database restore completed successfully."
else
    echo "Database restore failed."
fi
