#!/bin/bash

cd ..
docker-compose up -d

#until docker run -it postgres --add-host host.docker.internal:host-gateway \
#    docker.io/postgres:14-alpine -h localhost -p 5433 -U sophia pg_isready
./scripts/wait-for-it.sh -t 4 -h localhost -p 5433
#until docker run -it --rm --add-host host.docker.internal:host-gateway \
#do
#  echo "Waiting for postgres"
#  sleep 2;
#done

echo "docker is now running"
docker-compose down
