# Graphql Server

## MongoDB data

MongoDB data(include structure & documents) file is `/data/budshome.sql`.

If you need mongodb cloud count, email to me or wechat(微信): yupen-com, please.

## Build & run

``` Bash
git clone https://github.com/zzy/tide-async-graphql-mongodb.git
cd tide-async-graphql-mongodb
cargo build

cd backend
```

Rename file `.env.example` to `.env`, or put the environment variables into a `.env` file:

```
ADDRESS=127.0.0.1
PORT=8000

GRAPHQL_PATH=graphql
GRAPHIQL_PATH=graphiql

MONGODB_URI=mongodb://mongo:mongo@127.0.0.1:27017
MONGODB_BUDSHOME=budshome

SITE_KEY=0F4EHz+1/hqVvZjuB8EcooQs1K6QKBvLUxqTHt4tpxE=
CLAIM_EXP=10000000000
```

## Build & Run:

``` Bash
cargo run
```

GraphiQL: connect to http://127.0.0.1:8000/graphiql with browser.

![Graphql Image](../data/graphql.jpg)

## Queries

- getUserByEmail(...): User!
- getUserByUsername(...): User!
- userSignIn(...): SignInfo!
- allUsers(...): [User!]!
- allProjects: [Project!]!
- allProjectsByUser(...): [Project!]!

## MUTATIONS

- userRegister(...): User!
- userChangePassword(...): User!
- userUpdateProfile(...): User!
- addProject(...): Project!

## Sample Usage

Sample mutation for user register:
```
mutation {
  userRegister(
    newUser: { 
      email: "example@budshome.com", 
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

Sample query for user sign in:
```
{
  userSignIn(
    userAccount: {
      email: "example@budshome.com"
      username: ""
      password: "wo#$shi^$shui"
    }
  ) {
    email
    username
    token
  }
}
```

When submit method `userSignIn`, a token would be generated, use this token for query all users and every user's projects:
```
{
  allUsers(
    token: "fyJ0eXAiOiJKV1Q..."
  ) {
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

Sample query and mutation for projects was similar to users.

## Contributing

You are welcome in contributing to this project.
