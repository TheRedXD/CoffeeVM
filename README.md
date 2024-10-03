# CoffeeVM
CoffeeVM is a docker management software, built to be easy to host in userspace, without granting access to an entire docker instance.
So, anywhere where docker is available, CoffeeVM should be able to run perfectly fine. It's also made to be easy to integrate anywhere
and with anything, such that you don't have to lose your mind trying to either do things the scuffed way (via docker's CLI), using
docker compose or some other weird solution - CoffeeVM gives you an easy websocket interface to manage all of this, and deploy docker
containers easily.

If you're just looking to host things with docker without any integrations, this is probably not for you.
Look into [docker compose](https://docs.docker.com/compose/), or if you need a web panel, [portainer](https://www.portainer.io/) instead.

## ⚠️ WARNING!
As of writing this (2024-10-03), this rewrite of CoffeeVM (yes this is a rewrite) in Rust is very unfinished and lots of things need to be done!
It will probably contain many bugs and is currently not suitable for production use. This warning will be removed once it is determined that it
is stable.

You're welcome to contribute via making a Pull Request, or an Issue if you find any. Look in todo.md for more information.

## Building
To build the backend, simply open a terminal in the `backend` folder and run `cargo build --release`. This assumes you have Rust. If you do not, install
instructions are [here](https://www.rust-lang.org/tools/install).
To build the frontend, simply open a terminal in the `frontend` folder and run `yarn build`. If you don't have yarn, install it with `npm i -g yarn`,
or just use your own desired package manager (npm, pnpm, yarn or bun recommended, but you can generally use anything you prefer).