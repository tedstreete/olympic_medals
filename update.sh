#!/bin/sh

MIN_DELAY=$((60 * 60))    # 60 minutes in seconds
MAX_DELAY=$((90 * 60))    # 90 minutes in seconds

while true; do
  cargo run
    git add contents.txt
    git add index.html
    git add Medal-Records.csv
    git commit -m "Routine medals update"
    git push
    echo "Update medals count at $(date)"
    
    DELAY_MINUTES=$(($RANDOM%($MAX_DELAY-$MIN_DELAY+1)+$MIN_DELAY))
    echo "Waiting for $DELAY_MINUTES minutes before next medals table update ."
    sleep $DELAY_MINUTES

done