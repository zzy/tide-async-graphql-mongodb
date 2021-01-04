# tide-async-graphql-mongodb

**NOT** complete - Clean boilerplate for graphql services using tide, async-graphql, handlebars-rust, and mongodb.

## Features

- [x] User: query & mutation
- [x] Project: query & mutation
- [ ] User register
- [ ] Sign up & Sign in
- [ ] Encrypt password & Change password
- [ ] Profile Update
- [ ] JSON web token authentication

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

Sample mutation for user:
```
mutation {
  addUser(newUser:{email:"lllll@teddds222t.com", username:"李四"}) {
    id
    email
    username
  }
}
```

Sample query for projects:
```
{
  allProjects {
    id
    userId
    subject
    website
  }
}
```

Sample query for projects by user_id:
```
{
  allProjectsByUser(userId: "5ff317fd005050e5006a2474") {
    id
    userId
    subject
    website
  }
}
```

Sample mutation for project:
```
mutation {
  addProject(
    newProject: {
      userId: "5ff2fe82000c701100e10de4"
      subject: "测试项目3"
      website: "https://budshome.com"
    }
  ) {
    id
    userId
    subject
    website
  }
}
```
