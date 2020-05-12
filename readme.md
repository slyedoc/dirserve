# Directory Serve (dirserve)

A simple rust app that host the current directory to the web

> _Note_: This project was something simple to test using rust with vue and hopefully get something useful as well.

## Ideal Use case

Sometimes you want to host a file on your machine so someone else or you can quickly access from another device (phone, tablet, or just aother machine).  It would nice to able to though it up on webserver and access it, but that can be more setup and not something you really want up for long.

This is a binary you can run and it will host your currrent directory with manybe a few options.

## Command Line Options

- [x] Port
- [ ] Time ( how long to keep web server open)

## Release Build Steps

> Each block assumes your at root of proejct

Build frontend first

```(bash)
cd frontend
npm install
npm run build
```

Build backend

```(bash)
cd backend
cargo build --release
```

Copy dirserve to bin

```(bash)
sudo cp backend/target/release/dirserve /usr/local/bin
```
