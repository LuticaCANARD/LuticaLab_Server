# 테스트용 클라이언트 코드 
# 단순히 연결을 주고 벋는지만 테스트함.
from socket import *;

ip = "localhost" # IP address of the server
port = 8080 # Port number of the server

client = socket(AF_INET, SOCK_STREAM) # Create a socket object
client.connect((ip, port)) # Connect to the server

print("Connected to the server")


client.send("Hello, server\n".encode()) # Send a message to the server
client.send("Hello, server2\n".encode()) # Send a message to the server
client.send("Hello, server3".encode()) # Send a message to the server


client.close() # Close the connection