#!/bin/bash

DEFAULT_PATH="/dev/sdb1"
# Use the provided argument or default to the default path
SOURCE_PATH="${1:-$DEFAULT_PATH}"
TARGET_PATH="/var/www/media/uploads"
SERVICE_NAME="media.service"
DB_NAME="media"
SQL_DUMP_FILE="dump.sql"
JSON_FILE="delete_files.json"

# Function to handle errors
error_exit() {
    echo "Error: $1" >&2
    exit 1
}

source ./cp_files.bash
source ./dump.bash
source ./delete_files.bash

cp_files
dump
delete_files

echo "Successfully done"
