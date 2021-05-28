/*
 Source Server Type    : MongoDB
 Source Server Version : 40404
*/


// ----------------------------
// Collection structure for projects
// ----------------------------
db.getCollection("projects").drop();
db.createCollection("projects");

// ----------------------------
// Documents of projects
// ----------------------------
db.getCollection("projects").insert([ {
    _id: ObjectId("5ff318c100e6569b006a2475"),
    "user_id": ObjectId("5ff82b2c0076cc8b00e5cddb"),
    subject: "测试项目",
    website: "https://budshome.com"
} ]);
db.getCollection("projects").insert([ {
    _id: ObjectId("5ff318ee00436493006a2476"),
    "user_id": ObjectId("5ff82b2c0076cc8b00e5cddb"),
    subject: "测试项目2",
    website: "https://budshome.com"
} ]);
db.getCollection("projects").insert([ {
    _id: ObjectId("5ff319120023102e006a2477"),
    "user_id": ObjectId("5ff83f4b00e8fda000e5cddc"),
    subject: "测试项目3",
    website: "https://budshome.com"
} ]);

// ----------------------------
// Collection structure for users
// ----------------------------
db.getCollection("users").drop();
db.createCollection("users");

// ----------------------------
// Documents of users
// ----------------------------
db.getCollection("users").insert([ {
    _id: ObjectId("5ff82b2c0076cc8b00e5cddb"),
    email: "ok@budshome.com",
    username: "我谁24ok32",
    cred: "bOCMU4h9aO4o6jP3cujPDQi+b3Ig6JhZcLPWFpqbnog="
} ]);
db.getCollection("users").insert([ {
    _id: ObjectId("5ff83f4b00e8fda000e5cddc"),
    email: "oka@budshome.com",
    username: "我s谁24ok32",
    cred: "+d/XvT9JE/zjt6R/IAkwwwpk8q6y2Jhv3EvRH/UHEYE="
} ]);
db.getCollection("users").insert([ {
    _id: ObjectId("5ffd710400b6b84e000349f8"),
    email: "oka2@budshome.com",
    username: "我2s谁24ok32",
    cred: "801v0TV0h1XGMmCVTwItWjieGhI4hrGRAgNcutK8IW0="
} ]);
db.getCollection("users").insert([ {
    _id: ObjectId("5ffdb3fa00bbdf3a007a2988"),
    email: "afasf@budshome.com",
    username: "哈哈",
    cred: "P38V7+1Q5sjuKvaZEXnXQqI9SiY6ZMisB8QfUOP91Ao="
} ]);
db.getCollection("users").insert([ {
    _id: ObjectId("600b7a2700e7c21500d6cc1e"),
    email: "oka22@budshome.com",
    username: "我22s谁24ok32",
    cred: "fF7BBhTWbTGmC4Tu1rcw93D5S+G57WeDtzVmQjz0jro="
} ]);
db.getCollection("users").insert([ {
    _id: ObjectId("600b7a5300adcd7900d6cc1f"),
    email: "iok@budshome.com",
    username: "我是ok",
    cred: "peCwspEaVw3HB05ObIpnGxgK2VSQOCmgxjzFEOY+fk0="
} ]);
db.getCollection("users").insert([ {
    _id: ObjectId("600b8064000a5ca30024199e"),
    email: "iok2@budshome.com",
    username: "我是ok2",
    cred: "SJs0tA07rSN+4AHvLuN9zgZkihqJ+5no+lSax8DR8uE="
} ]);
db.getCollection("users").insert([ {
    _id: ObjectId("60894ddb00dac15e00911d8b"),
    email: "ooo@ooo.com",
    username: "哈哈哈",
    cred: "P38V7+1Q5sjuKvaZEXnXQqI9SiY6ZMisB8QfUOP91Ao="
} ]);
db.getCollection("users").insert([ {
    _id: ObjectId("608954d900136b6c0041ae09"),
    email: "budshome@budshome.com",
    username: "我是谁",
    cred: "P38V7+1Q5sjuKvaZEXnXQqI9SiY6ZMisB8QfUOP91Ao="
} ]);
