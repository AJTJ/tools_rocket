A simple CRUD server using Rocket

###SERVER SETUP
#####Rocket 
web framework
https://rocket.rs/v0.5-rc/guide/

#####Juniper 
for GraphQL
https://graphql-rust.github.io/juniper/current/quickstart.html

#####Diesel 
for Database interactions
https://diesel.rs/

excellent video tutorial
https://www.youtube.com/watch?v=8xhYXMrDYxk

###DATABASE SETUP
#####PostgreSQL
Running the official official docker image.
https://hub.docker.com/_/postgres

to run:

    docker run --name postgres -e POSTGRES_PASSWORD=mysecretpassword -d -p 5432:5432 postgres

cli access:
```
docker exec -it postgres bash 
root@cb9222b1f718:/# psql -U postgres
```

excellent docker/postgres resource
https://betterprogramming.pub/connect-from-local-machine-to-postgresql-docker-container-f785f00461a7


#####pgadmin4
Using the desktop app or the official image to interact with the db

To run:

    docker run -e PGADMIN_DEFAULT_EMAIL=email -e PGADMIN_DEFAULT_PASSWORD=password -d -p 8000:8000 dpage/pgadmin4
