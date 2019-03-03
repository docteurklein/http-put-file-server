# http PUT and GET file server

## What ?

An simple http server built in rust with rocket.rs.

## Why ?

To allow uploading and reading arbitrary large files located at arbitrary paths by streaming them end-to-end.

## How ?

Upload `$FILE` to the server and name it with a random UUID of my chosing in a folder named after me:

    curl -T $FILE "localhost:8000/$(whoami)/$FILE"

Get the file:

    curl "localhost:8000/$(whoami)/$FILE" -o $FILE
