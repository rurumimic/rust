# Run

## Podman

### Build & Run

```bash
podman build -t backyard:0.0.1 .
podman run --name backyard backyard:0.0.1 backyard
podman logs backyard
```

```bash
I'm growing Asparagus!
```

```bash
podman rm backyard
```

