#!/bin/bash

dump() {
	echo "Stopping service $SERVICE_NAME..."
	if ! systemctl stop "$SERVICE_NAME"; then
		error_exit "Failed to stop $SERVICE_NAME"
	fi

	echo "Restoring database $DB_NAME from $SQL_DUMP_FILE..."
	if ! psql -d "$DB_NAME" -f "$SQL_DUMP_FILE"; then
		error_exit "Failed to restore database $DB_NAME from $SQL_DUMP_FILE"
	fi

	echo "Starting service $SERVICE_NAME..."
	if ! systemctl start "$SERVICE_NAME"; then
		error_exit "Failed to start $SERVICE_NAME"
	fi
}