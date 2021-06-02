# tide-async-graphql-mongodb

Clean boilerplate for graphql services using tide, rhai, async-graphql, surf, graphql-client, yew, handlebars-rust, jsonwebtoken, and mongodb.

See also: https://github.com/zzy/surfer

## Features

- Graphql Services
  - [x] User register
  - [x] Salt and hash a password with PBKDF2 - 使用 PBKDF2 对密码进行加密（salt）和散列（hash）运算
  - [x] Sign in
  - [x] JSON web token authentication - JWT 鉴权整合
  - [x] Change password
  - [x] Profile Update
  - [x] User: query & mutation
  - [x] Project: query & mutation
- Web Application
  - [x] Client request, bring & parse GraphQL data
  - [x] Render data to template engine
  - [x] Define custom helper with Rhai scripting language

## Stacks

- [Rust](https://www.rust-lang.org) - [中文资料集萃](https://budshome.com)
- [Tide](https://crates.io/crates/tide) - [中文文档](https://tide.budshome.com)
- [rhai](https://crates.io/crates/rhai) - Embedded Scripting for Rust
- [async-graphql](https://crates.io/crates/async-graphql) - [中文文档](https://async-graphql.budshome.com)
- [mongodb & mongo-rust-driver](https://crates.io/crates/mongodb)
- [Surf](https://crates.io/crates/surf)
- [yew](https://crates.io/crates/yew)
- [graphql_client](https://crates.io/crates/graphql_client)
- [handlebars-rust](https://crates.io/crates/handlebars)
- [jsonwebtoken](https://crates.io/crates/jsonwebtoken)
- [cookie-rs](https://crates.io/crates/cookie)

## MongoDB data

MongoDB data(include structure & documents) file is `/data/budshome.sql`.

If you need mongodb cloud count, email to me or wechat(微信): yupen-com, please.

## How to run?

Please read:

- [**Frontend-handlebars: web application server**](./frontend-handlebars/README.md)
- [**Frontend-yew: web application server**](./frontend-yew/README.md)
- [**Backend: graphql servies server**](./backend/README.md)

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
