# abstract

This is a clean architecture actix-web project.

- Just lottery hero for HOK, [the old project for this](https://github.com/weiraneve/hok-lottery-actix)

- I hava kotlin SpringBoot project as some logic [hok-lottery](https://github.com/weiraneve/hok-lottery)

## apis

See [the API documentation pages](./apis) for more info.

## sql

See [lottery sql info](./sql) for more sql info

## usages

1. Create database and tables, you can see it by [lottery sql](./sql).
   this can be done in the terminal, then input your mysql root password:
   ```shell
   mysql -u root -p lottery < ./sql/lottery.sql
   ```
   
2. Create a `.env` file in this directory:
    ```ini
    SERVER_ADDR=127.0.0.1:8034
    DATABASE_URL=mysql://<username>:<password>@localhost:3306/lottery
    ```

3. Run the server

## architecture overview

my actix `Onion Architecture`:

```
src
├── api
│   ├── controllers
│   └── dto
├── domain
│   ├── models
│   ├── repositories
│   └── services
├── infrastructure
│   └── repositories
├── services
├── tests
│   └── api
└── main.rs
```

## testing support

To run the tests, you can go `src/tests` and use the following command:
```bash
cargo test
```
