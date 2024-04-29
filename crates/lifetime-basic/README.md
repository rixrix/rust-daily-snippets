# Lifetimes

Lifetimes are a way of tracking the scope of a reference to an object in memory.
In Rust, every value as one owner, when the owner goes out of scope, the value is dropped, its memory is freed.

Lifetimes allow Rust to ensure that a reference to an object is valid for as long as it's needed.
