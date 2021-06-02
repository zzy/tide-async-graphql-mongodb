# Web Application Server - handlebars

``` Bash
git clone https://github.com/zzy/tide-async-graphql-mongodb.git
cd tide-async-graphql-mongodb
cargo build

cd frontend-handlebars
```

Rename file `.env.example` to `.env`, or put the environment variables into a `.env` file:

```
ADDRESS=127.0.0.1
PORT=3000

GRAPHQL_PORT=8000
GRAPHQL_PATH=graphql
GRAPHIQL_PATH=graphiql
```

## Build & Run:

``` Bash
cargo run
```
Then connect to http://127.0.0.1:3000 with browser.

![Client Image](../data/client.jpg)

## How to Test & Run `rhai scripts`

You could use `rhai-repl` to test your rhai code, and use `rhai-run` to run it. `rhai-repl.rs` and `rhai-run.rs` are in the folder `frontend/scripts`, please copy them into `frontend/examples` folder, then test or run rhai code with command:

``` bash 
cargo run --example <rhai-repl>/<rhai-run ./scripts/script_to_run.rhai>
``` 

If you would want to install the rhai tool, use the command 

``` bash
cargo install --path . --example <rhai-repl>/<rhai-run>
```

then test rhai code using `rhai-repl`, and run scripts using the `rhai-run`:

``` bash
rhai-run ./scripts/script_to_run.rhai
```

## Contributing

You are welcome in contributing to this project.
