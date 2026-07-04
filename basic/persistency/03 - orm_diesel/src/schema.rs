// @generated automatically by Diesel CLI.
#![allow(warnings)]

diesel::table! {
    clients (id) {
        id -> Integer,
        name -> Varchar,
        phone -> Varchar,
    }
}
