## Development

To run automatically, install Docker and run:

```
docker network create caddy
docker compose -f docker-compose.local.yml  up --build -d

```


This will start your services as close to production as possible, just not available at the normal domain but on localhost.

Docker will eat up all your disk space with build cache, so you might want to clean it up from time to time. To do so, run 
```
docker system prune
``` 
and confirm with `y`.


## To delete unused Docker images and containers, you can use the following commands:

```
docker image prune -a
docker container prune

```

## Without docker
To run manually, install dependencies and run:

```
cd frontend
pnpm
pnpm dev
```

I like to use pnpm, but yarn or npm should work as well. To install pnpm, run `npm install -g pnpm`, which you might need superuser-rights for.

Rust API

Install Rust: https://www.rust-lang.org/learn/get-started

Development

```
cd api
cargo watch -x run
```

Production

```
cd api
cargo run --release
```

Of course, you can also install dependencies with npm (then the start script is npm run dev). To install yarn, run `npm install -g yarn`, which you might need superuser-rights for.