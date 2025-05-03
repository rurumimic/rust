# City

```bash
cargo run --bin server_seoul
cargo run --bin client_seoul
```

```bash
cargo run --bin server
cargo run --bin client_seoul
cargo run --bin client_tokyo
```

## Test

### gRPCurl

#### Install gRPCurl

```bash
go install github.com/fullstorydev/grpcurl/cmd/grpcurl@latest
```

#### Reflection

##### Service List

```bash
grpcurl -plaintext '[::1]:50051' list
```

```bash
city.newyork.City
city.seoul.City
city.tokyo.City
grpc.reflection.v1.ServerReflection
```

##### Describe Service

```bash
grpcurl -plaintext '[::1]:50051' describe
```

```bash
city.seoul.City is a service:
service City {
  rpc Process ( .city.seoul.SeoulRequest ) returns ( .city.seoul.SeoulResponse );
}
city.newyork.City is a service:
service City {
  rpc Process ( .city.newyork.NewYorkRequest ) returns ( .city.newyork.NewYorkResponse );
}
city.tokyo.City is a service:
service City {
  rpc Process ( .city.tokyo.TokyoRequest ) returns ( .city.tokyo.TokyoResponse );
}
grpc.reflection.v1.ServerReflection is a service:
service ServerReflection {
  rpc ServerReflectionInfo ( stream .grpc.reflection.v1.ServerReflectionRequest ) returns ( stream .grpc.reflection.v1.ServerReflectionResponse );
}
```

##### Call Reflection Service

```bash
grpcurl -d '{"list_services": ""}' \
-plaintext '[::1]:50051' \
grpc.reflection.v1.ServerReflection/ServerReflectionInfo
```

```json
{
  "originalRequest": {
    "listServices": ""
  },
  "listServicesResponse": {
    "service": [
      {
        "name": "city.seoul.City"
      },
      {
        "name": "city.newyork.City"
      },
      {
        "name": "city.tokyo.City"
      },
      {
        "name": "grpc.reflection.v1.ServerReflection"
      }
    ]
  }
}
```

#### Call gRPC server

##### Seoul

```bash
grpcurl \
-d '{"resident_id": 1, "district": "Gangnam"}' \
-plaintext '[::1]:50051' \
city.seoul.City/Process
```

```json
{
  "granted": true,
  "message": "Hello!"
}
```

##### Tokyo

`bytes payload` is base64 encoded:

```bash
echo -n -e '\x02\x03\x07' | base64

AgMH
```

```bash
grpcurl \
-d '{"signal": 3, "payload": "AgMH"}' \
-plaintext '[::1]:50051' \
city.tokyo.City/Process
```

```json
{
  "ack": true,
  "duration": "1"
}
```

