import socket

HOST = '0.0.0.0'
PORT = 4444

with socket.socket() as server:
    server.bind((HOST, PORT))
    server.listen(1)
    print(f"[+] Listening on {PORT}...")

    conn, addr = server.accept()
    print(f"[+] Connection from {addr}")

    with conn:
        while True:
            data = conn.recv(10240).decode().strip()
            print(f"[>] Received: {data}")

            if data.lower() == "EXIT":
                print("[-] Disconnecting ...")
                break
            if data.lower() == "HELLO":
                while True:
                    command_file = input("Enter Command Path > ").strip()
                    if len(command_file) <= 0:
                        print("Invalid input")
                        continue
                    import os.path;
                    if not os.path.exists(command_file):
                        print("File not found: "+command_file)
                        continue

                    print("[+] Sending ELF file...")
                    with open(command_file, 'rb') as f:
                        while True:
                            chunk = f.read(4096)
                            if not chunk:
                                break
                            conn.sendall(chunk)
                    print("[+] File sent successfully.")
            else:
                print(data)

    print("[*] Done.")
