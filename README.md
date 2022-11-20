# Serde Email

A validating email type that can be serialised using Serde.

## Introduction

This crate is for creating `Email` objects.

 * It allows you to have Email **as a type**. i.e. `let emails : Vec<Email> = vec![]`.
 * The `Email` type guarantees to be **validated**. Once it is created, you can be confident it's safe to use as an email.
 * The `Email` type can also be **used as strings**. This allows interoptability with lots of connector functions which will take a String.
 * It **supports Serde**. Serialisation with CLIs, requests, etc. Should all work thanks to this.

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
