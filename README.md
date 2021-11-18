# Discord 챗봇 개발 테스트 코드 저장소

### Dev stack

- rust
- tokio (async)

### deploy

1. heroku project setting

```
heroku create --buildpack emk/rust
heroku buildpacks:set emk/rust
```

2. heroku 사이트에서 git repository와 연결, deployment 설정 및 build

3. heroku에서 bot 실행 key, value 설정

4. create Procfile

```
worker: ./target/release/[project name]
```

5. source code git push

### 참조

- [heroku에 배포하기](https://github.com/emk/heroku-buildpack-rust)
- [Serenity 라이브러리 활용](https://www.youtube.com/watch?v=5k8sw_BgpwU&list=PLzIwronG0sE5lQCPFP69Ukgz4d9dngaSi&index=2)
