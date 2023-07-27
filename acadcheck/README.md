`acadcheck` is a simple to use, flexible library for building checkers used
in programming contests or homeworks. This crate is meant to act as a base,
avoiding the need of writing different scripts from scratch for different
types of usage.

# __Installation__

```toml
[dependencies]
acadnet = "0.1.0"
```

# __Features__

* `serde` for serialisation of outputs and errors.

```toml
acadnet = { version = "0.1.0", features = ["serde"] }
```