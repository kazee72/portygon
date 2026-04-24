# Portygon

A fast, lightweight TCP port scanner written in Rust.

## Features

- Asynchronous scanning with up to 100 concurrent connections
- Banner grabbing — detects service info on open ports
- HTTP-aware banner parsing — extracts status line and Server header
- DNS resolution — accepts hostnames or IP addresses as targets
- Stealth mode — sequential scan with randomised delays (2–5s)
- JSON output for scripting and automation

## Installation

```bash
git clone https://github.com/kazee72/portygon.git
cd portygon
cargo build --release
```

The binary will be at `target/release/portygon`.

## Usage

```
portygon <TARGET> [OPTIONS]
```

### Options

| Flag | Default | Description |
|---|---|---|
| `-p, --ports` | `1-1024` | Ports to scan — single, range, or comma-separated |
| `-s, --stealth` | off | Sequential scan with random 2–5s delays |
| `-j, --json` | off | Output results as JSON |
| `-V, --version` | — | Print version |

### Examples

Scan default ports (1–1024) on a host:
```bash
portygon 192.168.1.1
```

Scan specific ports:
```bash
portygon 192.168.1.1 -p 22,80,443
```

Scan a port range:
```bash
portygon 192.168.1.1 -p 1-10000
```

Mix of ranges and individual ports:
```bash
portygon 192.168.1.1 -p 22,80,443,8000-8080
```

Use a hostname:
```bash
portygon example.com -p 80,443
```

JSON output:
```bash
portygon 192.168.1.1 -p 1-1024 --json
```

Stealth mode:
```bash
portygon 192.168.1.1 -p 80,443 --stealth
```

### Sample Output

```
[Open Ports]
22: SSH-2.0-OpenSSH_8.9
80: HTTP/1.1 200 OK | Server: nginx/1.18.0
443: HTTP/1.1 200 OK | Server: nginx/1.18.0
```

JSON mode:
```json
{
  "target": "192.168.1.1",
  "ports_scanned": 1024,
  "open_ports": [
    { "port": 22, "banner": "SSH-2.0-OpenSSH_8.9" },
    { "port": 80, "banner": "HTTP/1.1 200 OK | Server: nginx/1.18.0" }
  ]
}
```

## Running Tests

```bash
cargo test
```

Slow tests (stealth mode, large ranges) are marked `#[ignore]` and can be run explicitly:

```bash
cargo test -- --ignored
```