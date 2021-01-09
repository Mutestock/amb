
![Rust](https://github.com/Mutestock/amb/workflows/Rust/badge.svg)
![Deploy](https://github.com/Mutestock/amb/workflows/Deploy/badge.svg)
![Node.js CI](https://github.com/Mutestock/amb/workflows/Node.js%20CI/badge.svg)

Guide:

For development:

docker-compose up --build

Need these environment variables:

PG_USER
PG_PASS
PG_DB
DATABASE_URL
AMBIENCE_JWT_TOKEN_SECRET
DATABASE_API_KEY

Alternatively edit the docker compose files with some hard coded values of your own.
Alternatively use an .env file (untested)

You will be able to monitor the database with software like pgadmin or DBeaver


Although the program will create a resources folder itself, the created folder might require root privileges on linux, which is an issue when writing files to it. It is currently advised to create the resources folder by yourself. Sorry for the inconvenience.  