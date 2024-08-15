# Build docker system

Each app will be build in a separate container. This will allow us to scale the app independently and update only the front or back end without affecting the other.

## Build the front end

```bash
podman build -t front:latest-dev -f build/front/Containerfile .
```

## Build the back end

```bash
podman build -t back:latest-dev -f build/back/Containerfile .
```
