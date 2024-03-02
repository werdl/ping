# `werdl/ping`
> ping rewrite in rust
## usage
```sh
ping <TARGET>
```
- target: the target to ping (includes port)
### example
```sh
ping google.com:443
```
- pings google.com on port 443
### options/flags
- `--count` or `-c`: the number of pings to send (option)
- `--interval` or `-i`: the interval between pings (option)
- `--timeout` or `-t`: the timeout for each ping (option)
- `--packet-size` or `-p`: the size of each ping (option)
- `--verbose` or `-v`: display verbose output (flag)
- `--help` or `-h`: display help
