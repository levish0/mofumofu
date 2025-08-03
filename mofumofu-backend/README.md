## JWT Secret Key
```bash
openssl rand -base64 32
```
- Generates a 32-byte (256-bit) random value encoded in Base64.
- Use the output as your JWT_SECRET in your .env file or environment variables.

# 🚀 Dockerfile 설명 (`mofumofu-backend`)
### Dockerfile
[Dockerfile](Dockerfile) 참고

### 빌드
```bash
docker build -t mofumofu-backend .
```

### container 실행
```
docker run -p 8000:8000 --env-file docker.env mofumofu-backend
```
### ghcr.io
```
docker-compose up
```
```
docker-compose start
```
```
docker-compose up --build
```