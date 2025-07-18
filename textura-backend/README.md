## JWT Secret Key
```bash
openssl rand -base64 32
```
- Generates a 32-byte (256-bit) random value encoded in Base64.
- Use the output as your JWT_SECRET in your .env file or environment variables.