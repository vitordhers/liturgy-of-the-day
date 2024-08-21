#!/bin/bash

export $(grep -v '^#' .env | xargs)

echo "Running command with SURREAL_DB_USER=$SURREAL_DB_USER, namespace=$SURREAL_DB_NS, database=$SURREAL_DB_DATABASE"
surreal sql --endpoint http://127.0.0.1:$SURREAL_DB_PORT --username $SURREAL_DB_USER --password $SURREAL_DB_PASSWORD --namespace $SURREAL_DB_NS --database $SURREAL_DB_DATABASE