use std::net::*;
pub mod socket_manager 
{
    // Socket들의 연결을 관리하고자 하는 객체.
}

struct UserConnection
{
    connection_id:i32,
    connect_socket:TcpListener
}
