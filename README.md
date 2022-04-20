# ISSR - Rust Tally Demo 

Interactive SSR (ISSR) is a WebSocket-based framework that allows you to write a declarative HTTP server in any language and have the client sync state automatically. See the [main readme](https://github.com/interactive-ssr/issr-server/blob/master/main.org) for more information.

This is the example project for how to use ISSR with the Rust web framework [Rocket](https://rocket.rs/). For an example with the [Axum](https://github.com/tokio-rs/axum) web framework, checkout the `axum` branch.

## Quickstart

```sh
# Clone the repository
$ git clone https://github.com/interactive-ssr/tally-demo-rust

# The default branch uses Rocket. For Axum, checkout the axum branch:
git checkout axum

# Start the Rocket or Axum server. This server will only be accessed internally by the ISSR server.
$ cd tally-demo-rust
$ cargo r

# In another terminal, start the ISSR server. This is the public-facing server that uses WebSockets to synchronize state.
$ docker pull charje/issr-server
$ # cd tally-demo-rust
$ docker compose up issr

# You should now be able to access the public-facing server at 192.168.1.1:3000/tally 
```
