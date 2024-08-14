### Deploy to local wasmCloud

## Setup

### wasmCloud

Install `wash` with `brew install wash`.

```bash
wash --version
# wash 0.30.0
```

run

```bash
wash up -d
```

### Build and sign the components

```bash
./build_and_sign.fish
```

### Deploy the components with `wadm`

```bash
./up.fish
```

### Delete the application with `wadm`

```bash
./down.fish
```
