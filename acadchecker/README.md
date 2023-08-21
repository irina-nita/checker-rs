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
       ]
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
 
 
