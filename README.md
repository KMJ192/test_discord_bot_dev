# Discord 챗봇 개발 테스트 코드 저장소

### Dev stack

- rust
- tokio (async)

### sernity

- https://discordnet.dev/api/Discord.EmbedImage.html

### 참조

- [heroku에 배포하기](https://github.com/emk/heroku-buildpack-rust)
- [Serenity 라이브러리 활용](https://morioh.com/p/1cb48da69b63)

### 프로젝트 설정 순서

1. Discord 개발자 포털에서 discord application 생성

- https://discord.com/developers/applications

2. Bot

- 메뉴 Bot에서 새로운 bot 생성 => Build-A-Bot

3. OAuth2

- URL Generator => bot 선택
- scopes에서 bot 선택하면 아래에 기능 선택화면이 나타난다.
- 기능을 선택하면 아래에 URL이 나타난다.
- URL을 웹 사이트로 접속
- 접속하면 서버에 봇을 추가할 수 있음

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
