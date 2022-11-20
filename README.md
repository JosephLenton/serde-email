# emailio

A validating email type that can be serialised

## Introduction

This crate is for creating `Email` objects.

 * It allows you to have Email **as a type**. i.e. `let emails : Vec<Email> = vec![]`.
 * The `Email` type guarantees to be **validated**. Once it is created, you can be confident it's safe to use as an email.
 * The `Email` type can also be **used as strings**. This allows interoptability with lots of connector functions which will take a String.
 * It **supports Serde**. Serialisation with CLIs, requests, etc. Should all work thanks to this.
