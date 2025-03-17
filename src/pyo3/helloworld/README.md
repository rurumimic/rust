# Hello World

## To install the Python shared library

```bash
# ubuntu
sudo apt install python3-dev

# fedora
sudo yum install python3-devel
```

## To create a new project

```bash
cargo new helloworld
cd helloworld
cargo add pyo3 -F auto-initialize
```

## Use python 3.10

```bash
uv venv -p 3.10
source .venv/bin/activate
```

## Run the project

```bash
cargo run
```

```bash
Hello User, I'm Python 3.10.12
```

