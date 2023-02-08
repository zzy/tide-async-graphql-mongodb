# tide-async-graphql-mongodb

Clean boilerplate for graphql services, wasm/yew frontend, handlebars frontend. 

Using tide, rhai, async-graphql, surf, graphql-client, yew, handlebars, jsonwebtoken, and mongodb. 

See also: https://github.com/zzy/surfer

Demo site:
- [niqin.com - NiQin Books Platform | 泥芹书馆](https://niqin.com)
- [piexue.com - Project Matchmaking | 项目对接](https://piexue.com)

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

- [Rust](https://www.rust-lang.org) - [中文资料集萃](https://niqin.com)
- [Tide](https://crates.io/crates/tide) - [中文文档](https://tide-book.niqin.com)
- [rhai](https://crates.io/crates/rhai) - Embedded Scripting for Rust
- [async-graphql](https://crates.io/crates/async-graphql) - [中文文档](https://async-graphql.niqin.com)
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

- [**Backend: graphql servies server**](./backend/README.md)
- [**Frontend-yew: web application server**](./frontend-yew/README.md)
- [**Frontend-handlebars: web application server**](./frontend-handlebars/README.md)

## Contributing

You are welcome in contributing to this project.
