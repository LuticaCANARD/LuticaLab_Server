# Channel

## 개념

- 채널이란, 메세지 전파를 위한 그룹입니다.
- 채널에 등록된 클라이언트들은 만약 채널로 메세지가 전달되면 모두 그 메세지를 전달받는다.

## 구현

### Channel 상세구현

- 이 서버에서는 stream으로 접근가능한 string의 vec로 구현합니다.

```rs
let channel = vec[
    "id1",
    "id2",...
]
```

### 메세지의 상세구현

- 메세지의 전달은 채널 벡터의 순회로 구현합니다.
