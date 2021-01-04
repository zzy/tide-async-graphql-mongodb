# tide-async-graphql-mongodb
Clean boilerplate for graphql services using tide, async-graphql, handlebars-rust, and mongodb.

- [Tide 中文文档](https://tide.budshome.com)
- [async-graphql 中文文档](https://async-graphql.budshome.com)

## Features

- [x] User: query & mutation
- [x] Project: query & mutation
- [ ] User register
- [ ] Sign up & Sign in
- [ ] Encrypt password & Change password
- [ ] Profile Update
- [ ] JSON web token authentication

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
