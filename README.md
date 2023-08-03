# acadcheck, acadchecker

`acadcheck` is a simple to use, flexible library for building checkers used
in programming contests or homeworks. This crate is meant to act as a base,
avoiding the need of writing different scripts from scratch for different
types of usage.

# __Installation__

```toml
[dependencies]
acadcheck = "0.1.6"
```

# __Features__

* `serde` for serialisation of outputs and errors.

```toml
acadnet = { version = "0.1.6", features = ["use-serde"] }
```
------------------------------------------------------

 `acadchecker` is a CLI tool for building checkers used in programming contests or homeworks.
 The checker is configured from a json file.
 
 # Installation
 
 ```shell
 cargo install acadchecker
 ```
 
 # Usage
 
 ```shell
 acadchecker --config config.json
 ```
 
 # __Config Example__
 ```json
 {
   "checker": {
     "monitors": [
       {
         "time": {
           "limit": {
             "secs": 5,
             "nanos": 0
           }
         }
       }
     ],
     "output_type": {
       "scored": {
         "per_test": 5
       }
     },
     "in_refs": {
       "1": [
         "/binary/tests/in/001.in",
         "/binary/tests/ref/001.ref"
       ],
     }
   },
   "processor": {
     "gcc": {
       "language": "c++",
       "flags": [
         "-Werror",
         "-Wall"
       ],
       "exec": "/binary/solution"
     }
   },
   "solution": {
     "file": "/binary/solution.cpp"
   },
   "out_dir": "/binary/tests/out",
   "security": {
     "user": "sandbox",
     "group": "restricted"
   }
 }
 
 ```
 