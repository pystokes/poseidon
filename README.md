# poseidon

Dialogue engine

## Docker

```shell
docker build -t poseidon .
```

```shell
docker run -it --rm -v `pwd`:/app -p 80:8000 poseidon
```

## Brief usage

Run application.

```bash
cd poseidon
cargo run
```

Access from browser.

```text
http://{hostname}:80/hello/{name}
```
