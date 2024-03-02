# `werdl/ping`
> ping rewrite in rust
- as opposed to the original `ping` command, which round-trips to the target, this command sends a single garbage (all 1) packet to the target, without waiting for a response
## usage
```sh
ping <TARGET>
```
- target: the target to ping (includes port)
### example
```sh
ping google.com
```
- pings google.com on port 80, the default port
### options/flags
- `--count` or `-c`: the number of pings to send (option, default until interrupted)
- `--interval` or `-i`: the interval between pings (option, default 1s)
- `--timeout` or `-t`: the timeout for each ping (option, default 1s)
- `--packet-size` or `-p`: the size of each ping (option, default 64 bytes)
- `--verbose` or `-v`: display verbose output (flag, default false)
- `--help` or `-h`: display help
## output
- note that the output is not the same as the original `ping` command
- we say failed, as opposed to lost, to indicate that the ping failed, because they may have timed out, _or_ been lost
### `ping google.com:443`
```sh
PING 142.250.200.46:443 (google.com:443) 64(64) bytes of data. # what we are pinging
64 bytes to 142.250.200.46:443: icmp_seq=0 time=9.226 ms # ping response
64 bytes to 142.250.200.46:443: icmp_seq=1 time=9.949 ms
64 bytes to 142.250.200.46:443: icmp_seq=2 time=10.982 ms
64 bytes to 142.250.200.46:443: icmp_seq=3 time=9.649 ms
64 bytes to 142.250.200.46:443: icmp_seq=4 time=9.951 ms
64 bytes to 142.250.200.46:443: icmp_seq=5 time=9.487 ms
64 bytes to 142.250.200.46:443: icmp_seq=6 time=10.309 ms
64 bytes to 142.250.200.46:443: icmp_seq=7 time=10.678 ms
64 bytes to 142.250.200.46:443: icmp_seq=8 time=23.024 ms
64 bytes to 142.250.200.46:443: icmp_seq=9 time=10.486 ms
64 bytes to 142.250.200.46:443: icmp_seq=10 time=8.924 ms
^C # user interrupt
--- google.com:443 ping statistics --- # ping statistics
11 packets transmitted, 11 received, 0.000% failed # data
connection and sending min/avg/max/stddev = 8.924/11.151/23.024/3.801 ms # data
```
- what is stddev?
  - standard deviation
  - a measure of the amount of variation or dispersion of a set of values
  - a low standard deviation means that the values tend to be close to the mean of the set, while a high standard deviation means that the values are spread out over a wider range
  - in this case, it is the amount of variation in the connection and sending times
### `ping reddit.com -t 0.01`
```sh
PING 151.101.1.140:80 (reddit.com) 64(64) bytes of data. # what we are pinging
64 bytes to 151.101.1.140:80: icmp_seq=0 time=10.473 ms (timeout by 0.473ms) # ping response, which timed out
64 bytes to 151.101.1.140:80: icmp_seq=1 time=8.587 ms
64 bytes to 151.101.1.140:80: icmp_seq=2 time=8.618 ms
64 bytes to 151.101.1.140:80: icmp_seq=3 time=8.876 ms
64 bytes to 151.101.1.140:80: icmp_seq=4 time=9.738 ms
64 bytes to 151.101.1.140:80: icmp_seq=5 time=8.952 ms
64 bytes to 151.101.1.140:80: icmp_seq=6 time=10.501 ms (timeout by 0.501ms)
64 bytes to 151.101.1.140:80: icmp_seq=7 time=9.860 ms
64 bytes to 151.101.1.140:80: icmp_seq=8 time=8.184 ms
64 bytes to 151.101.1.140:80: icmp_seq=9 time=9.287 ms
64 bytes to 151.101.1.140:80: icmp_seq=10 time=9.342 ms
64 bytes to 151.101.1.140:80: icmp_seq=11 time=10.257 ms (timeout by 0.257ms)
64 bytes to 151.101.1.140:80: icmp_seq=12 time=10.547 ms (timeout by 0.547ms)
64 bytes to 151.101.1.140:80: icmp_seq=13 time=10.171 ms (timeout by 0.171ms)
64 bytes to 151.101.1.140:80: icmp_seq=14 time=9.257 ms
64 bytes to 151.101.1.140:80: icmp_seq=15 time=10.626 ms (timeout by 0.626ms)
64 bytes to 151.101.1.140:80: icmp_seq=16 time=8.766 ms
64 bytes to 151.101.1.140:80: icmp_seq=17 time=8.796 ms
64 bytes to 151.101.1.140:80: icmp_seq=18 time=8.471 ms
64 bytes to 151.101.1.140:80: icmp_seq=19 time=10.013 ms (timeout by 0.013ms)
64 bytes to 151.101.1.140:80: icmp_seq=20 time=9.878 ms
64 bytes to 151.101.1.140:80: icmp_seq=21 time=10.182 ms (timeout by 0.182ms)
64 bytes to 151.101.1.140:80: icmp_seq=22 time=9.087 ms
64 bytes to 151.101.1.140:80: icmp_seq=23 time=9.235 ms
64 bytes to 151.101.1.140:80: icmp_seq=24 time=9.582 ms
64 bytes to 151.101.1.140:80: icmp_seq=25 time=10.379 ms (timeout by 0.379ms)
64 bytes to 151.101.1.140:80: icmp_seq=26 time=10.427 ms (timeout by 0.427ms)
64 bytes to 151.101.1.140:80: icmp_seq=27 time=9.861 ms
64 bytes to 151.101.1.140:80: icmp_seq=28 time=9.323 ms
64 bytes to 151.101.1.140:80: icmp_seq=29 time=8.061 ms
64 bytes to 151.101.1.140:80: icmp_seq=30 time=8.785 ms
64 bytes to 151.101.1.140:80: icmp_seq=31 time=8.665 ms
64 bytes to 151.101.1.140:80: icmp_seq=32 time=10.497 ms (timeout by 0.497ms)
64 bytes to 151.101.1.140:80: icmp_seq=33 time=9.042 ms
^C
--- reddit.com ping statistics ---
34 packets transmitted, 34 received, 32.353% failed # response data 
connection and sending min/avg/max/stddev = 8.061/9.480/10.626/0.751 ms
```