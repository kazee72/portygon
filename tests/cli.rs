use assert_cmd::Command;
use predicates::prelude::*;
use std::sync::OnceLock;

struct ServerGuard {
    child: std::process::Child,
}

impl Drop for ServerGuard {
    fn drop(&mut self) {
        self.child.kill().ok();
    }
}

fn start_server() -> ServerGuard {
    let child = std::process::Command::new("python")
        .arg("test_server/server.py")
        .spawn()
        .expect("failed to start test server");

    std::thread::sleep(std::time::Duration::from_millis(500));

    ServerGuard { child }
}

static SERVER: OnceLock<ServerGuard> = OnceLock::new();

fn setup() {
    SERVER.get_or_init(|| start_server());
}

#[test]
fn test_all_ports_open() {
    setup();

    Command::cargo_bin("portygon")
        .unwrap()
        .args(&["127.0.0.1", "-p", "8080,8888,9001,9002"])
        .assert()
        .success()
        .stdout(predicate::str::contains("8080"))
        .stdout(predicate::str::contains("8888"))
        .stdout(predicate::str::contains("9001"))
        .stdout(predicate::str::contains("9002"));
}

#[test]
fn test_http_banner_parsing() {
    setup();

    Command::cargo_bin("portygon")
    .unwrap()
    .args(&["127.0.0.1", "-p", "8080"])
    .assert()
    .success()
    .stdout(predicate::str::contains("HTTP/1.1 200 OK | Server: portygon-test-server"));
}

#[test]
fn test_normal_banner_parsing() {
    setup();

    Command::cargo_bin("portygon")
    .unwrap()
    .args(&["127.0.0.1", "-p", "9001"])
    .assert()
    .success()
    .stdout(predicate::str::contains("Connection successful | Server: portygon-test-server"));
}

#[test]
fn test_json_output() {
    setup();

    Command::cargo_bin("portygon")
        .unwrap()
        .args(&["127.0.0.1", "-p", "8080", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"target\""))
        .stdout(predicate::str::contains("\"ports_scanned\""))
        .stdout(predicate::str::contains("\"open_ports\""))
        .stdout(predicate::str::contains("\"127.0.0.1\""));
}

#[test]
fn test_invalid_ip_exits_with_error() {
    Command::cargo_bin("portygon")
        .unwrap()
        .args(&["notanip", "-p", "8080"])
        .assert()
        .failure()
        .code(1);
}

#[test]
fn test_empty_port_list_exits_with_error() {
    Command::cargo_bin("portygon")
        .unwrap()
        .args(&["127.0.0.1", "-p", "invalid"])
        .assert()
        .failure()
        .code(1);
}

#[test]
fn test_hostname_resolution() {
    setup();

    Command::cargo_bin("portygon")
        .unwrap()
        .args(&["localhost", "-p", "8080,8888,9001,9002"])
        .assert()
        .success()
        .stdout(predicate::str::contains("8080"))
        .stdout(predicate::str::contains("8888"))
        .stdout(predicate::str::contains("9001"))
        .stdout(predicate::str::contains("9002"));
}

#[test]
#[ignore]
fn test_stealth_mode() {
    setup();

    Command::cargo_bin("portygon")
        .unwrap()
        .args(&["127.0.0.1", "-p", "8080,8888", "--stealth"])
        .assert()
        .success()
        .stdout(predicate::str::contains("8080"))
        .stdout(predicate::str::contains("8888"));
}

#[test]
#[ignore]
fn test_large_port_range() {
    Command::cargo_bin("portygon")
        .unwrap()
        .args(&["127.0.0.1", "-p", "1-1024"])
        .assert()
        .success();
}