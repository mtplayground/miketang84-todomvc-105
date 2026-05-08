# miketang84-todomvc-105

TodoMVC built with Leptos and Axum SSR.

## Docker

Build the multi-stage image:

```bash
docker build -t miketang84-todomvc-105 .
```

Run it with a mounted SQLite data volume at `/data` and the required environment variables:

```bash
docker run --rm \
  -p 8080:8080 \
  -v todomvc-data:/data \
  -e DATABASE_URL=sqlite:///data/todomvc.db \
  -e LEPTOS_SITE_ADDR=0.0.0.0:8080 \
  -e RUST_LOG=info,tower_http=info \
  miketang84-todomvc-105
```
