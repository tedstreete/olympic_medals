#!/bin/sh

cargo run
git add contents.txt
git add index.html
git add Medal-Records.csv
git commit -m "Routine medals update"
git push