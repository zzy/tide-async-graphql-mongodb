# tide-async-graphql-mongodb

**NOT** complete - Clean boilerplate for graphql services using tide, async-graphql, handlebars-rust, and mongodb.

## Features

- [x] User: query & mutation - 用户查询和变更
- [x] Project: query & mutation - 项目查询和变更
- [x] User register - 用户注册
- [x] Salt and hash a password with PBKDF2 - 使用 PBKDF2 对密码进行加密（salt）和散列（hash）运算
- [ ] Sign in & Sign out - 签入和注销
- [ ] Change password - 修改密码
- [ ] Profile Update - 资料更新
- [ ] JSON web token authentication - JWT 整合

## Stacks

- [Rust](https://www.rust-lang.org)
- [Tide](https://github.com/http-rs/tide) - [中文文档](https://tide.budshome.com)
- [async-graphql](https://crates.io/crates/async-graphql) - [中文文档](https://async-graphql.budshome.com)
- [mongodb & mongo-rust-driver](https://crates.io/crates/mongodb)
- [handlebars-rust](https://crates.io/crates/handlebars)
- [jsonwebtoken](https://crates.io/crates/jsonwebtoken)

## How to run?

``` Bash
git clone https://github.com/zzy/tide-async-graphql-mongodb.git
cd tide-async-graphql-mongodb
```

Put the environment variables into a `.env` file:

```
WEB_ADDRESS=0.0.0.0
WEB_PORT=8080

GRAPHQL_ADDRESS=0.0.0.0
GRAPHQL_PORT=8080
GRAPHQL_PATH=graphql
GRAPHIQL_PATH=graphiql

MONGODB_URI=mongodb://mongo:mongo@localhost:27017
MONGODB_BUDSHOME=budshome
```

Build & Run:

``` Bash
cargo build
cargo run
```

GraphiQL: connect to http://localhost:8080/graphiql with browser.

## Usage

Sample query for all users:
```
{
  allUsers {
    id
    email
    username 
  }
}
```

Sample query for all users and every user's projects:
```
{
  allUsers {
    id
    email
    username 
    
    projects {
      id
      userId
      subject
      website
    }
  }
}
```

Sample mutation for user register:
```
mutation {
  userRegister(
    newUser: { 
      email: "whoami@budshome.com", 
      username: "我是谁", 
      password: "wo#$shi^$shui" 
    }
  ) {
    id
    email
    username
  }
}

```

Sample query and mutation for projects was similar to users.
