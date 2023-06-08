# This file contains the implementation of Dragon Bot Z Character Service's 
# Database docker image.
#
# Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>
FROM ubuntu:22.04

# Sets up the system
RUN apt-get update && apt-get install -y \
    dirmngr \
    ca-certificates \
    software-properties-common \
    gnupg \
    gnupg2 \
    apt-transport-https \
    curl

# Imports PostgreSQL repository
RUN curl -fSsL https://www.postgresql.org/media/keys/ACCC4CF8.asc | gpg --dearmor | tee /usr/share/keyrings/postgresql.gpg > /dev/null

# Imports Stable build
RUN echo deb [arch=amd64,arm64,ppc64el signed-by=/usr/share/keyrings/postgresql.gpg] http://apt.postgresql.org/pub/repos/apt/ jammy-pgdg main | tee -a /etc/apt/sources.list.d/postgresql.list

# Installs PostgreSQL
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=Europe/Paris
RUN apt-get update && apt-get install -y \
    postgresql-client \
    postgresql

# Sets path to include postgres executable
ENV PATH="/usr/lib/postgresql/15/bin:${PATH}"

# Setup
WORKDIR /database
COPY init_database.sql .

# Changes the user to postgres
USER postgres

# Starts the server
RUN mkdir -p /var/lib/postgresql/data/
RUN initdb -D /var/lib/postgresql/data

# Copies config
COPY pg_hba.conf /var/lib/postgresql/data
COPY postgresql.conf /var/lib/postgresql/data

USER root

RUN chmod 777 /var/lib/postgresql/data/pg_hba.conf
RUN chmod 777 /var/lib/postgresql/data/postgresql.conf

USER postgres

# Starts the database
CMD ["postgres", "-D", "/var/lib/postgresql/data"]
