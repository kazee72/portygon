import socket, threading

ports = [8080, 8888, 9001, 9002]
http_ports = [80, 443, 8080, 8443, 8000, 8888, 3000, 3001, 5000, 5173, 4200, 8081, 9090, 9443]

def handle_port(port):
    s = socket.socket()
    s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    s.bind(("127.0.0.1", port))
    s.listen()

    while True:
        conn, addr = s.accept()

        if port in http_ports:
            conn.recv(1024)
            conn.sendall(b"HTTP/1.1 200 OK\r\nServer: portygon-test-server\r\n\r\n")
        else:
            conn.sendall(b"Connection successful | Server: portygon-test-server\n")

        conn.close()
        
    s.close()



threads = []
for port in ports:
    t = threading.Thread(target=handle_port, args=(port,))
    threads.append(t)
    t.start()

for t in threads:
    t.join()
