# Subscribe Group에 관한 정의

## 총체적 개념

- 이하의 설계서는 TCP/IP의 5단계 전반의 user control plane의 설계임.

### BigFlow

- 연결이 들어오면 그 연결을 id와 함께 서버에 등록함.
- 등록을 원하는 그룹이 제시되면 자격 검증을 진행한 후 자격이 올바르면 그룹에 등재.
- 등재된 소켓은 필요에 따라서 호출될 수 있음.
- 메세지 호출에 따라서 그룹별로 메세지를 보냄.

## 과정 상세

### 등재

- 소켓을 서버에 인식시키고, 그 소켓을 호출자와 소켓간의 연결을 매칭시킴.

#### 등재자격 증명 Auth

- 등재 자격을 가지고 있는지는 암호화된 초기 연결메시지를 통하여 증명함.

#### 등재 후 통신

- 등재가 완료되고 나면, `accept_loop`에 정의된 `connection_loop`의 `broker_handle`를 타고 `match event`구문을 통해 Message의 Type을 구분한다.
- 그 다음, 연결이 `connection_loop`에서 수립되면, `broker`를 통하여 `SocketObject::Connection(connection)`를 보내준다.
- 이후 다음 명령이 올때까지 기다리고(`.next().await{...}`), 다음 명령이 오면 읽어서 `broker`를 통해 message를 제어한다.

#### 호출자의 정의

- 호출자는 호출자 생성 메세지에 정의된 정보를 활용해 생성한다.
- 중복 key가 존재하는 경우를 대비하여 생성된 id를 return한다.

### 그룹 메시지 송신

- 

#### Global Message 송신



#### 특정 채널 송신


