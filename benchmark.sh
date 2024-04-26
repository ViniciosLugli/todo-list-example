#!/bin/bash
URL="http://0.0.0.0:3000"
USERNAME="ViniciosLugli"
PASSWORD="murilinho"

# Base64 encoding for Basic Auth
AUTH=$(echo -n "$USERNAME:$PASSWORD" | base64)

# Variables to track total duration
total_duration_user=0
total_duration_task=0
total_duration_get=0

# Function to create a new user
function create_user() {
  for i in {1..1000}; do
    DATA='{"username":"user'$i'","password":"password1"}'
    START_TIME=$(date +%s%3N)
    curl -s -X POST "$URL/users" -H "Content-Type: application/json" -d "$DATA" -o /dev/null
    END_TIME=$(date +%s%3N)
    DURATION=$((END_TIME-START_TIME))
    total_duration_user=$((total_duration_user+DURATION))
  done
  echo "Total Duration for creating users: $total_duration_user ms"
}

# Function to get all tasks
function get_tasks() {
  for i in {1..1000}; do
    START_TIME=$(date +%s%3N)
    curl -s -X GET "$URL/tasks" -H "Authorization: Basic $AUTH" -o /dev/null
    END_TIME=$(date +%s%3N)
    DURATION=$((END_TIME-START_TIME))
    total_duration_get=$((total_duration_get+DURATION))
  done
  echo "Total Duration for getting tasks: $total_duration_get ms"
}

# Function to create a new task
function create_task() {
  for i in {1..1000}; do
    DATA='{"title":"New Task '$i'","description":"Description of new task"}'
    START_TIME=$(date +%s%3N)
    curl -s -X POST "$URL/tasks" -H "Content-Type: application/json" -H "Authorization: Basic $AUTH" -d "$DATA" -o /dev/null
    END_TIME=$(date +%s%3N)
    DURATION=$((END_TIME-START_TIME))
    total_duration_task=$((total_duration_task+DURATION))
  done
  echo "Total Duration for creating tasks: $total_duration_task ms"
}

# Execute functions to send requests concurrently
create_user &
create_task &
get_tasks &

# Wait for all background jobs to finish
wait

echo "All requests have been sent and processed."
