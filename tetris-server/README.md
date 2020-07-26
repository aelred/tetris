The server hosts the high-scores - they're stored in a JSON file and the 
server will confirm their validity when new ones are received.

Because it uses Rocket, it requires nightly (for now...):

```bash
$ rustup override set nightly
$ ROCKET_PORT=4444 cargo run
  ...
🚀 Rocket has launched from http://localhost:4444
```