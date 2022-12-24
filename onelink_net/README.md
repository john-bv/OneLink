# Onelink Net

The network library for onelink. Please note it mainly contains protocol not the server to host the protocol, for that you need to use `onelink::net::Server`

#### Connection Handshake

1. (C -> S) **Connection Request.** You send a packet containing the following information to the server.
   
   - `version`: The protocol version of onelink you are using. This is a short.
   
   - `credentials`: The information you give to the server to identify yourself.
   
   - `device_id`: The type of device trying to connect. This is assigned by onelink. If this is a new connection, this is a guest id.
   
   - `encrypt`: Whether or not you, as the user, wish to encrypt traffic. The server will only respect this value if the server allows unsafe traffic.
   
   - `mtu`: The Maximum transfer unit you would like to use. 

2. (S -> C) **Connection Reply**. You will recieve this packet AFTER sending a connection request (given that the connection was granted, if not, you will recieve a Disconnect packet). The packet contains the following fields:
   
   -  `duration`: The maximum amount of time you are allowed to access this server (in seconds). If this is set to `0` you are allowed indefinitely. This is usually set to prevent load on large instances with bad hardware and generally will not affect the performance of onelink.
   
   - `mtu`: The MTU size the server is allowing you to use. This is NEVER higher than the MTU you set; on official servers! However, this may be LOWER than the MTU size you're requesting, in which case MUST be respected! Or the server will disconnect you.
   
   - `heart_ack`: The acknowledge sequence. This will define how long you should send a heartbeat to the server.

3. T
