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
