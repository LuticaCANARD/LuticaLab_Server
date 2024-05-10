# 테스트용 클라이언트 코드 
# 단순히 연결을 주고 벋는지만 테스트함.
from socket import *;
import time
ip = "localhost" # IP address of the server
port = 8080 # Port number of the server

client = socket(AF_INET, SOCK_STREAM) # Create a socket object
client.connect((ip, port)) # Connect to the server


print("Connected to the server")
client.send("HI\n".encode()) # Send a message to the server

id = client.recv(1024) # Receive a message from the server
print(id)

for i in range(100):
    client.send("Hello, server NEXT...".encode()) # Send a message to the server
    time.sleep(1)

client.close() # Close the connection
