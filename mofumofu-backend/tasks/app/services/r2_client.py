import boto3
from botocore.config import Config
from app.core.config import settings
import logging

logger = logging.getLogger(__name__)

class R2Client:
    def __init__(self):
        self.account_id = settings.R2_ACCOUNT_ID
        self.access_key_id = settings.R2_ACCESS_KEY_ID
        self.secret_access_key = settings.R2_SECRET_ACCESS_KEY
        self.bucket_name = settings.R2_BUCKET_NAME
        self.public_domain = settings.R2_PUBLIC_DOMAIN
        
        # Cloudflare R2 엔드포인트 설정
        endpoint_url = f"https://{self.account_id}.r2.cloudflarestorage.com"
        
        # S3 클라이언트 설정 (R2는 S3 호환)
        self.s3_client = boto3.client(
            's3',
            endpoint_url=endpoint_url,
            aws_access_key_id=self.access_key_id,
            aws_secret_access_key=self.secret_access_key,
            region_name='auto',
            config=Config(
                signature_version='s3v4',
                retries={'max_attempts': 3}
            )
        )
    
    async def upload_file(self, key: str, file_data: bytes, content_type: str = None) -> str:
        """파일을 R2에 업로드하고 공개 URL 반환"""
        try:
            extra_args = {}
            if content_type:
                extra_args['ContentType'] = content_type
            
            # 파일 업로드
            self.s3_client.put_object(
                Bucket=self.bucket_name,
                Key=key,
                Body=file_data,
                **extra_args
            )
            
            # 공개 URL 생성
            public_url = f"{self.public_domain}/{key}"
            logger.info(f"파일이 R2에 업로드됨: {public_url}")
            return public_url
            
        except Exception as e:
            logger.error(f"R2 업로드 실패: {str(e)}")
            raise
    
    async def delete_file(self, key: str) -> bool:
        """R2에서 파일 삭제"""
        try:
            self.s3_client.delete_object(
                Bucket=self.bucket_name,
                Key=key
            )
            logger.info(f"파일이 R2에서 삭제됨: {key}")
            return True
        except Exception as e:
            logger.error(f"R2 삭제 실패: {str(e)}")
            return False
    
    async def file_exists(self, key: str) -> bool:
        """파일이 R2에 존재하는지 확인"""
        try:
            self.s3_client.head_object(
                Bucket=self.bucket_name,
                Key=key
            )
            return True
        except Exception:
            return False

# 전역 R2 클라이언트 인스턴스
r2_client = R2Client()