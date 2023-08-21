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
 # __mw__

## 1. Docs
```
.
├── Cargo.toml
├── .dev.container
│   ├── .config
│   │   └── .env
│   └── docker-compose.yaml
├── Dockerfile
├── README.md
├── sandbox.json
├── secret
│   ├── decrypt_env.sh
│   └── env.enc
└── src
    ├── api
    │   ├── mod.rs
    │   └── utils.rs
    ├── main.rs
    └── sandbox
        ├── mod.rs
        └── tests.rs
```

> `TODO`

## 2. API
 
### `POST /submission/run`

##### Parameters (multipart form)

|          type        |    key                     | required   |
|--------------------- |----------                  |----------  |
|   `file`             | `solution`                 | _true_     |
|   `application/json` | [`config`](#config.json)   | _true_     |
|   `text`             | `problem`                  | _true_     |
##### Responses

 | http code     | content-type                      | response                                                            |
 |---------------|-----------------------------------|---------------------------------------------------------------------|
 | `200`         | `application/json`                | [`output`](#output.json)                                            |
 | `500`         | `application/json`                | [`error_response`](#error_response.json)                            |

##### Example cURL

```javascript
curl -X POST \
     -F problem="50" \
     -F solution=@sol.cpp \
     -F config='{ "processor": { "gcc": { "language": "c++", "flags": [ "-Wall", "-Werror" ] } }, "time_limit": { "secs": 5 } };type=application/json' \
     http://localhost:4000/submission/run
```

##### JSON examples

###### config.json
```json
{
  "processor": {
    "gcc": {
      "language": "c++",
      "flags": [
        "-Wall",
        "-Werror"
      ]
    }
  },
  "time_limit": {
    "secs": 5
  }
}
```
###### output.json
```
{
    "results": {
        "0": {
            "failed": "Expected: 726372166\r\n\nBut got: 9438934\n"
        },
        "1": {
            "passed"
        },
        "2": {
            "failed": "Expected: 702909132\r\n\nBut got: 1003939\n"
        }
    }
}
```
> or
```
{
    "error": "Compilation failed: exit status: 1"
}
```
###### error_response.json
```
{
    "message": "NoSuchKey: The specified key does not exist."
}
```

------------------------------------------------------------------------------------------

### `GET /healthcheck/aws`

##### Responses

 | http code     | content-type                      | response                                                            |
 |---------------|-----------------------------------|---------------------------------------------------------------------|
 | `200`         | `application/json`                | `"There are 142 objects in bucket."`                                |
 | `417`         | `application/json`                |  `"Did not pass healtcheck: ..."`                                   |
 | `500`         | `application/json`                |    `"AWS Client not provided: ..."`                                 |

##### Example cURL

```javascript
curl -X GET \
     http://localhost:4000/healthcheck/aws
```

------------------------------------------------------------------------------------------

### `GET /healthcheck/docker`

##### Responses

 | http code     | content-type                      | response                                                            |
 |---------------|-----------------------------------|---------------------------------------------------------------------|
 | `200`         | `application/json`                | [`docker_health`](#docker_health.json)                              |
 | `417`         | `application/json`                |   `"Did not pass healtcheck: ..."`                                  |


###### docker_health.json
```
{
    "Containers": 266,
    "Images": 464,
    "Driver": "overlay2",
    "DockerRootDir": "/var/snap/docker/common/var-lib-docker",
    "DriverStatus": [
        [
            "Backing Filesystem",
            "extfs"
        ],
        [
            "Supports d_type",
            "true"
        ],
        [
            "Native Overlay Diff",
            "true"
        ],
        [
            "userxattr",
            "false"
        ]
    ],
    "ID": "",
    "KernelVersion": "",
    "MemTotal": 004544,
    "MemoryLimit": true,
    "NCPU": 4,
    "NEventsListener": 1,
    "NGoroutines": 88,
    "Name": "",
    "OperatingSystem": "Ubuntu Core 22",
    "SwapLimit": true,
    "SystemTime": "+03:00"
}
```

##### Example cURL

```javascript
curl -X GET \
     http://localhost:4000/healthcheck/docker
```

