# zero-to-rust

```shell
$ docker run -d postgres --name newsletter -p 5432:5432 -e POSTGRES_PASSWORD={password}  postgres -N 1000

docker run -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=Password123 -e POSTGRESS_DB=newsletter -p 5432:5432 -d postgres postgres -N 1000
```