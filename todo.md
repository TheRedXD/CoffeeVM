# TODO:
This is a list of things to do for CoffeeVM (and some things that already have been done)
- [x] Initialize the frontend
- [x] Basic backend connectivity (back when backend was written in Node.js, now this has to be ported to Rust)
- [ ] Fully port backend to Rust
- [x] Host structure and implementation
- [x] Container creation
- [x] Container start/stop
- [x] Container stdin
- [ ] Container stdout
- [ ] Handle CTRL-C (press once to stop all containers gracefully, again to kill them)
- [ ] Make proper web backend with a websocket endpoint, probably using axum and tower-http
- [ ] Create a system performance measuring file thingamabob (system.rs), such that we can measure CPU usage, memory usage, etc.
- [ ] Create proper volume management (volume.rs)
- [ ] Get the web interface working with the backend properly - such that you can create containers, view them and manage them.
- [ ] [BurgerPanel](https://github.com/TheBlueBurger/BurgerPanel) Integration Plugin