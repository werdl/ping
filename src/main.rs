//! A executable designed to replicate the functionality of the `ping` command. Also supports hostname resolution.

use dns_lookup::lookup_host;

use std::{io::Error as IoError, net::IpAddr};

use clap::Parser;

#[derive(Debug)]
struct Ip {
    address: String,
    hostname: String,
    port: u16,

    ping_options: PingOptions,
}

impl Ip {
    fn new(ping_options: PingOptions) -> Result<Ip, IoError> {
        let skim = Ip::hostname_skim(ping_options.target.as_str());

        let address = lookup_host(skim.0.as_str());

        let address = match address {
            Ok(address) => address,
            Err(_) => {
                return Err(IoError::new(
                    std::io::ErrorKind::AddrNotAvailable,
                    format!("Could not resolve hostname: {}", ping_options.target),
                ))
            }
        };

        let address = match address.get(0) {
            Some(address) => address,
            None => {
                return Err(IoError::new(
                    std::io::ErrorKind::AddrNotAvailable,
                    "No address found",
                ))
            }
        };

        let hostname = address.to_string();
        Ok(Ip {
            address: address.to_string(),
            port: skim.1,
            hostname,
            ping_options,
        })
    }

    fn hostname_skim(hostname: &str) -> (String, u16) {
        /*
           valid hostnames:
           - www.google.com
           - google.com
           - http://www.google.com
           - https://www.google.com
           - www.google.com:80
           - www.google.com:443
           - www.google.com:8080
           - www.google.com:65535
           - 142.250.200.14
           - 142.250.200.14:80
        */

        // first, check if the hostname is an ip address (ipv4 or ipv6)
        if hostname.parse::<IpAddr>().is_ok() {
            let port = match hostname.find(':') {
                Some(index) => {
                    let port = &hostname[index + 1..];
                    match port.parse::<u16>() {
                        Ok(port) => port,
                        Err(_) => 80,
                    }
                }
                None => 80,
            };

            return (hostname.to_string(), port);
        }

        let mut hostname = hostname.to_string();

        let port = match hostname.find(':') {
            Some(index) => {
                let port = &hostname[index + 1..];
                match port.parse::<u16>() {
                    Ok(port) => port,
                    Err(_) => 80,
                }
            }
            None => 80,
        };

        // now we have the port, we must remove any protocol
        hostname = hostname.replace("http://", "").replace("https://", "");

        // now we have the hostname, we must remove the port
        hostname = match hostname.find(':') {
            Some(index) => hostname[..index].to_string(),
            None => hostname,
        };

        (hostname, port)
    }

    fn ping(&self) {
        println!(
            "PING {} ({}) {}({}) bytes of data.",
            self.hostname,
            self.address,
            self.ping_options.packet_size.unwrap_or(56),
            self.ping_options.packet_size.unwrap_or(56)
        );

        // ping the tcp address, with 56 bytes of garbage data
        // print whether or not the connection goes through
        // print the time it took to get a response

        let packet_size = self.ping_options.packet_size.unwrap_or(56);
        let count = self.ping_options.count.unwrap_or(4);
        let timeout = self.ping_options.timeout.unwrap_or(4.0);
        let interval = self.ping_options.interval.unwrap_or(1.0);

        for i in 0..count {
            // ping
            let start = std::time::Instant::now();

            let ip_addr = self.address.parse::<IpAddr>();

            let ip_addr = match ip_addr {
                Ok(ip_addr) => ip_addr,
                Err(err) => {
                    println!("Error: {}", err);
                    std::process::exit(1);
                }
            };

            let socket_addr =
                std::net::SocketAddr::new(ip_addr, self.port);

            // now wait for the connection to finish, or timeout
            let result = std::net::TcpStream::connect_timeout(
                &socket_addr,
                std::time::Duration::from_secs_f64(timeout),
            );

            let duration = start.elapsed().as_millis();

            let failed = result.is_err();

            match result {
                Ok(_) => {
                    println!(
                        "{} bytes from {}: icmp_seq={} time={} ms",
                        packet_size, self.address, i, duration
                    );
                }
                Err(err) => {
                    println!("Request timeout for icmp_seq {}", i);
                }
            }

            if (duration as f64) > timeout * 1000.0  && !failed {
                println!("Request timeout for icmp_seq {}", i);
            }

            std::thread::sleep(std::time::Duration::from_secs_f64(interval));
        }
    }

    fn printiv(&self, message: &str) {
        if self.ping_options.verbose {
            println!("{}", message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hostname_skim() {
        let (hostname, port) = Ip::hostname_skim("www.google.com");
        assert_eq!(hostname, "www.google.com");
        assert_eq!(port, 80);

        let (hostname, port) = Ip::hostname_skim("google.com");
        assert_eq!(hostname, "google.com");
        assert_eq!(port, 80);

        let (hostname, port) = Ip::hostname_skim("http://www.google.com");
        assert_eq!(hostname, "www.google.com");
        assert_eq!(port, 80);

        let (hostname, port) = Ip::hostname_skim("https://www.google.com");
        assert_eq!(hostname, "www.google.com");
        assert_eq!(port, 80);

        let (hostname, port) = Ip::hostname_skim("www.google.com:80");
        assert_eq!(hostname, "www.google.com");
        assert_eq!(port, 80);

        let (hostname, port) = Ip::hostname_skim("www.google.com:443");
        assert_eq!(hostname, "www.google.com");
        assert_eq!(port, 443);

        let (hostname, port) = Ip::hostname_skim("www.google.com:8080");
        assert_eq!(hostname, "www.google.com");
        assert_eq!(port, 8080);

        let (hostname, port) = Ip::hostname_skim("www.google.com:65535");
        assert_eq!(hostname, "www.google.com");
        assert_eq!(port, 65535);
    }
}

#[derive(Debug, Parser)]
struct PingOptions {
    target: String,

    #[clap(short, long)]
    count: Option<usize>,

    #[clap(short, long)]
    timeout: Option<f64>,

    #[clap(short, long)]
    packet_size: Option<usize>,

    #[clap(short, long)]
    interval: Option<f64>,

    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let opts = PingOptions::parse();

    let ip = match Ip::new(opts) {
        Ok(ip) => ip,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };

    ip.ping();
}
