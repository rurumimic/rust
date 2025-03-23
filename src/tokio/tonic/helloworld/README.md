# Hello World

- tonic: [helloworld](https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md)

```bash
cargo new helloworld
cd helloworld
```

```bash
cargo add tonic
cargo add prost
cargo add tokio -F macros -F rt-multi-thread
cargo add --build tonic-build
```

## Run

### Run a server

```bash
cargo run --bin server
```

```bash
Got a request: Request { metadata: MetadataMap { headers: {"content-type": "application/grpc", "user-agent": "grpcurl/dev-build (no version set) grpc-go/1.61.0", "te": "trailers", "grpc-accept-encoding": "gzip"} }, message: HelloRequest { name: "Tonic" }, extensions: Extensions }
```

### Run a client

```bash
cargo run --bin client

RESPONSE=Response { metadata: MetadataMap { headers: {"content-type": "application/grpc", "date": "Sun, 23 Mar 2025 07:24:31 GMT", "grpc-status": "0"} }, message: HelloReply { message: "Hello Tonic!" }, extensions: Extensions }
```

## grpcurl

- github: [fullstorydev/grpcurl](https://github.com/fullstorydev/grpcurl)

### Install grpcurl

```bash
go install github.com/fullstorydev/grpcurl/cmd/grpcurl@latest
```

### Run grpcurl

```bash
grpcurl -plaintext \
-import-path ./proto \
-proto helloworld.proto \
-d '{"name": "Tonic"}' \
'[::1]:50051' helloworld.Greeter/SayHello
```

```json
{
  "message": "Hello Tonic!"
}
```

