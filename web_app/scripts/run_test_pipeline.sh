#!/bin/bash

# move to directory of the project
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH
cd ..

# spin up docker and hold script until accepting connections
docker-compose up -d
until pg_isready -h localhost -p 5433 -U sophia
do
  echo "Waiting for postgres"
  sleep 2;
done

# build our Rust server
# and run unit tests
cargo build
cargo test

# run the server in background
cargo run config.yml &
SERVER_PID=$!
# be sure the server is ready to accept connections
sleep 5

# run our migrations
diesel migration run

# move back to the scripts directory ??

# create the user (thru postman>code generator>curl)
curl --location 'http://127.0.0.1:8000/v1/user/create' \
--header 'Content-Type: application/json' \
--data-raw '{
    "name": "daemon",
    "email": "daemon@sophia.org",
    "password": "test"
}'

# login getting a fresh token
echo $(curl --location --request GET 'http://localhost:8000/v1/auth/login' \
--header 'Content-Type: application/json' \
--data-raw '{
    "username": "daemon",
    "password": "test"
    }') > ./fresh_token.json

# insert the fresh token into the Newman data and run the Newman API tests
TOKEN=$(jq '.token' fresh_token.json)
jq '.auth.apikey[0].value = '"$TOKEN"'' \
ftodo_items.postman_collection.json > test_newman.json

newman run test_newman.json

# our testing is done. We can clean up  the files created when running the tests, destroy
# the docker container and stop our server running with the following code
rm ./fresh_token.json
rm ./test_newman.json

# shut down the Rust server
kill $SERVER_PID

cd ..
docker-compose down

