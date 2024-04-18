pub mod socket_manager 
{
    use std::net::*;
    use std::collections::HashMap;
    // Socket들의 연결을 관리하고자 하는 모듈.
    pub trait ConnectManager {
        fn regist (&mut self,target:TcpStream);
    }
    
    pub struct UserConnectionTCP
    {
        connection_id:i32,
        connect_socket:TcpStream
    }

    pub struct UserConnectionTCPManager
    {
        connections : HashMap<i32,UserConnectionTCP>,
        last_id : i32
    }

    impl ConnectManager for UserConnectionTCPManager {
        fn regist (&mut self,target:TcpStream ) {
            let conn = UserConnectionTCP
            {
                connection_id : self.last_id,
                connect_socket : target
            };
            self.connections.insert(self.last_id, conn);
            self.last_id += 1;
        }
    }
}

