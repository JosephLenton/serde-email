<div align="center">
  <h1>
    Serde Email
  </h1>

  <h2>
    A validating email type that can be serialised using Serde (and Sea Orm).
  </h2>

  [![crate](https://img.shields.io/crates/v/serde-email.svg)](https://crates.io/crates/serde-email)
  [![docs](https://docs.rs/serde-email/badge.svg)](https://docs.rs/serde-email)
</div>

## Introduction

This crate is for creating `Email` objects.

 * It allows you to have Email **as a type**. i.e. `let emails : Vec<Email> = vec![]`.
 * The `Email` type guarantees to be **validated**. Once it is created, you can be confident it's safe to use as an email.
 * The `Email` type can also be **used as strings**. This allows interoptability with lots of connector functions which will take a String.
 * It **supports Serde**. Serialisation with CLIs, requests, etc. Should all work thanks to this.

## Features

 * `serde` **Default** - Enables serde serialisation and deserialisation.
 * `sea-orm` - Enables Sea Orm use with DB entities.

## Usage

### Building your own email addresses

```rust
use ::emailio::Email;

let email = Email::new("test@example.com".to_string()).expect("A valid email address");
```

### Validating the email address yourself

```rust
use ::emailio::is_valid_email;

if is_valid_email(&"test@example.com") {
  // do something
}
```

### Serialisation / Deserialisation

```rust
use ::emailio::Email;
use ::serde_json;

struct Person {
  name: String,
  email: Email,
}

// Some JSON input data as a &str. Maybe this comes from the user.
let data = r#"
    {
        "name": "John Doe",
        "email": "john@example.com"
    }"#;

// Parse the string of data into serde_json::Value.
let person: Person = serde_json::from_str(data).unwrap();

// Access parts of the data by indexing with square brackets.
println!("Hello {} I'll email you are {}", person.name, person.email);
```

### Sea Orm Entities

You can use the `Email` type with Sea Orm, including using it to save data to the DB.
Underneath it will serialise to a `Text` type within the DB.

```rust
use ::sea_orm::entity::prelude::*;
use ::serde::Deserialize;
use ::serde::Serialize;
use ::serde_email::Email;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub email: Email, // use as an email field
    pub created: OffsetDateTime,
}
```

## Special Thanks

The validation is all done by the [email_address crate](https://crates.io/crates/email_address).
