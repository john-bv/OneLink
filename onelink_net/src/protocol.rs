pub struct Packet {
    id: u16,
    signed: bool,
    payload: Payload
}

/// Handshake begin
/// id: 0x01
pub struct ConnectRequest {
    /// The version of the protocol you're using
    pub version: u16,
    /// The credentials of the account trying to connect.
    /// Sometimes is a string of `username:password` however
    /// credentials may also be a JWT. (username:password) encoding
    /// is disabled on onelink official servers.
    pub credentials: String,
    /// The ID of the device you are using, this is
    /// assigned by one-link, use `WEB-{SESSION_ID}` for web.
    pub device_id: String,
    /// Whether or not you wish to encrypt network traffic
    /// Sometimes the server will set this to be always enabled,
    /// in this case, you will be disconnected.
    pub encrypt: bool,
    /// The maximum transfer unit you wish to recieve.
    /// In some cases, you may be required to pad your requests with this unit,
    /// in case of encryption, this is strongly advised.
    pub mtu: u16
}

/// Sent in response to `ConnectRequest`
pub struct ConnectReply {
    /// The maximum time this client may be connected (in seconds)
    /// If this is set to 0, you are indefinitely allowed, UNLESS a
    /// disconnect notification or permissible update is pushed
    pub duration: u32,
    /// The maxmium transfer unit the server accepted for you to use.
    /// Sometimes this is lower than what you requested, however it will
    /// never be higher.
    pub mtu: u16,
    /// The rate which the client should ACK to the server. (in milliseconds)
    pub heart_ack: u64
}