# Test Concept — Portygon

## 1. Introduction

This document describes the test planning, execution, and evaluation for **Portygon**, an asynchronous TCP port scanner written in Rust. The tests cover both functional correctness and performance behaviour.

---

## 2. Test Goals

| Goal | Description |
|---|---|
| Functional correctness | Ports are detected correctly, banners parsed correctly, error handling works |
| Performance | Scan duration relative to port count and concurrency |
| Robustness | Correct behaviour on invalid input |

---

## 3. Test Environment

| Parameter | Value |
|---|---|
| OS | Windows 11 |
| Rust | 1.88.0 (stable) |
| Build | Release (`cargo build --release`) |
| Test framework | `cargo test`, `assert_cmd` |
| Performance tool | `hyperfine` |
| Test server | Python 3, bound locally to `127.0.0.1` |

---

## 4. Test Data

### 4.1 Local Test Server

To ensure deterministic, network-independent tests, a local Python server was developed (`test_server/server.py`). It opens defined ports on `127.0.0.1` and returns controlled banners.

| Port | Type | Banner |
|---|---|---|
| 8080 | HTTP | `HTTP/1.1 200 OK \| Server: portygon-test-server` |
| 8888 | HTTP | `HTTP/1.1 200 OK \| Server: portygon-test-server` |
| 9001 | Plain | `Connection successful \| Server: portygon-test-server` |
| 9002 | Plain | `Connection successful \| Server: portygon-test-server` |

The server starts automatically before integration tests via `std::process::Command` and is shut down after completion by a `Drop` guard.

### 4.2 Scan Targets

| Target | Purpose |
|---|---|
| `127.0.0.1` | Primary target for all tests |
| `localhost` | Hostname resolution test |
| `notanip` | Error handling test |

---

## 5. Test Cases

### 5.1 Unit Tests

| ID | Function | Description | Expected Result |
|---|---|---|---|
| U01 | `parse_ports` | Single port | `[80]` |
| U02 | `parse_ports` | Comma-separated | `[80, 1024, 3000]` |
| U03 | `parse_ports` | Port range | `[80..86]` |
| U04 | `parse_ports` | Whitespace | Whitespace ignored |
| U05 | `parse_ports` | Invalid input | Empty Vec |
| U06 | `parse_ports` | Empty string | Empty Vec |
| U07 | `parse_ports` | Mixed valid/invalid | Valid ports extracted |
| U08 | `parse_ports` | Range + comma | Correct combination |
| U09 | `parse_banner` | Full HTTP response | Status line + Server header |
| U10 | `parse_banner` | No Server header | Status line only |
| U11 | `parse_banner` | Status line only | Status line |
| U12 | `parse_banner` | Empty string | Empty string |
| U13 | `parse_banner` | Unusual Server value | Parsed correctly |
| U14 | `parse_banner` | `ServerName` must not match | Only `Server:` matches |

### 5.2 Integration Tests

| ID | Test | Description | Expected Result |
|---|---|---|---|
| I01 | `test_all_ports_open` | All 4 ports detected | stdout contains 8080, 8888, 9001, 9002 |
| I02 | `test_http_banner_parsing` | HTTP banner parsed correctly | `HTTP/1.1 200 OK \| Server: portygon-test-server` |
| I03 | `test_normal_banner_parsing` | Plain banner returned correctly | `Connection successful \| Server: portygon-test-server` |
| I04 | `test_json_output` | JSON output correctly structured | Fields `target`, `ports_scanned`, `open_ports` present |
| I05 | `test_invalid_ip_exits_with_error` | Invalid IP → exit code 1 | Exit code 1 |
| I06 | `test_empty_port_list_exits_with_error` | Invalid ports → exit code 1 | Exit code 1 |
| I07 | `test_hostname_resolution` | Hostname resolved to IPv4 | Ports correctly detected |

### 5.3 Performance Tests (hyperfine)

| ID | Command | Metric |
|---|---|---|
| P01 | `portygon 127.0.0.1 -p 1-100` | Scan duration, variance |
| P02 | `portygon 127.0.0.1 -p 1-500` | Scan duration, scaling |
| P03 | `portygon 127.0.0.1 -p 1-1024` | Scan duration, scaling |
| P04 | `portygon 127.0.0.1 -p 8080` | Duration with 1 open port |
| P05 | `portygon 127.0.0.1 -p 8080,8888,9001,9002` | Duration with 4 open ports (concurrency proof) |

---

## 6. Test Results

### 6.1 Unit Tests

All 14 unit tests passed.

```
test result: ok. 14 passed; 0 failed
```

### 6.2 Integration Tests

All 7 integration tests passed (2 marked `#[ignore]` for slow tests).

```
test result: ok. 7 passed; 0 failed; 2 ignored
```

**Finding during development:** `localhost` resolved to `::1` (IPv6) on the test machine, while the test server was bound to `127.0.0.1` (IPv4) only. Fix: `resolve_target()` now prefers IPv4 addresses from DNS results, falling back to IPv6 if no IPv4 address is available.

### 6.3 Performance Tests

| ID | Mean | Min | Max | σ |
|---|---|---|---|---|
| P01 (100 ports) | 3.413s | 2.167s | 14.562s | ±3.918s |
| P02 (500 ports) | 10.296s | 10.268s | 10.337s | ±0.023s |
| P03 (1024 ports) | 22.394s | 22.370s | 22.410s | ±0.012s |
| P04 (1 open port) | 2.173s | 2.155s | 2.184s | ±0.009s |
| P05 (4 open ports) | 2.183s | 2.170s | 2.203s | ±0.011s |

---

## 7. Evaluation

### 7.1 Scaling Behaviour

Scan duration for closed ports scales approximately linearly with port count:

- 100 ports → ~3.4s (~1 timeout batch at 3s)
- 500 ports → ~10.3s (~3 timeout batches)
- 1024 ports → ~22.4s (~7 timeout batches)

The bottleneck is the **3 second connection timeout** per batch (semaphore limit: 100 concurrent connections), not CPU or network.

### 7.2 Concurrency Proof

P04 vs P05 clearly demonstrates that the async task model works correctly:

| | 1 port | 4 ports |
|---|---|---|
| Mean | 2.173s | 2.183s |
| Difference | — | +10ms |

4 ports in parallel take virtually the same time as 1 port. Total duration is dominated by the **2 second read timeout**, not port count — direct evidence of working parallelism.

### 7.3 Conclusion

Portygon behaves functionally correct in all tested scenarios. The primary performance factor is the timeout values in `scanner.rs`, not concurrency or network overhead. Reducing the connection timeout would proportionally decrease scan duration for closed ports, at the cost of reliability on slow hosts.