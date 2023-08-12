# Database
To setup the database, pull the postgres image and edit the `postgresql.conf` 
file as well as `pg_hba.conf` located at `/var/lib/postgresql/data/`.

## New port
To use a port which differs from the default port (5432), run the following 
command from within the container:
```bash
$ psql -p <port>
```
