## CA-CS-804-B Web Application Development

The project is done using React framework with Tailwind CSS for the frontend and Rust with Rocket for the API.

1. Frontend
I used a MVVM architecture to implement the frontend using React and Taliwind CSS. The foundation is up and running and after going into the frontend directory and running it, you can view the work I did on the https://localhost:3000.

2. API
The api is done using Rocket, which is a web framework for Rust that makes it simple to write fast, secure web applications without sacrificing flexibility, usability, or type safety.



## Installation

To run automatically, install Docker and run:

```
docker network create caddy
docker compose -f docker-compose.yml  up --build -d
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