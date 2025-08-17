## Image Upload Specifications

### Supported Formats
- **JPEG/JPG** - Automatically compressed and converted to WebP
- **PNG** - Automatically compressed and converted to WebP  
- **GIF** - Preserved as original format (no compression)
- **WebP** - Processed and optimized

### Image Dimensions & Compression
All images are automatically resized only if they exceed the maximum dimensions, maintaining aspect ratio:

- **Avatar Images**: 512 × 512 pixels maximum
- **Banner Images**: 1600 × 400 pixels maximum  
- **Post Thumbnails**: 800 × 450 pixels maximum
- **Post Images**: 2000 × 2000 pixels maximum

### File Size Limits
- **Avatar**: 4MB maximum
- **Banner**: 8MB maximum
- **Thumbnails**: 4MB maximum  
- **Post Images**: 8MB maximum

### Automatic Optimization
- Non-GIF images are converted to WebP format for better compression
- Images larger than maximum dimensions are resized using high-quality Lanczos3 algorithm
- Quality setting: 90 for optimal balance between file size and image quality

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