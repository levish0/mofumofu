import httpx
import asyncio
from celery import current_task
from app.core.celery_app import celery_app
from app.services.r2_client import r2_client
from app.services.db_service import db_service
import logging

logger = logging.getLogger(__name__)

# 지원되는 이미지 타입
SUPPORTED_IMAGE_TYPES = {
    "image/jpeg", "image/jpg", "image/png", "image/webp", "image/gif"
}

# 최대 이미지 크기 (8MB)
MAX_IMAGE_SIZE = 8 * 1024 * 1024

def is_supported_image_type(content_type: str) -> bool:
    """지원되는 이미지 타입인지 확인"""
    return content_type.lower() in SUPPORTED_IMAGE_TYPES

@celery_app.task(bind=True, name="upload_profile_image")
def upload_profile_image_task(self, user_id: str, image_url: str):
    """
    Google OAuth에서 받은 프로필 이미지를 R2에 업로드하는 Celery 태스크
    
    Args:
        user_id: 사용자 ID
        image_url: Google 프로필 이미지 URL
    
    Returns:
        dict: 성공 시 public_url, 실패 시 error 메시지
    """
    try:
        # 태스크 상태 업데이트
        current_task.update_state(
            state='PROGRESS',
            meta={'status': 'Google에서 이미지 다운로드 중...'}
        )
        
        # 비동기 함수를 동기적으로 실행
        result = asyncio.run(_download_and_upload_image_oauth(user_id, image_url))
        
        return {
            'status': 'SUCCESS',
            'public_url': result,
            'message': f'프로필 이미지가 성공적으로 업로드되었습니다: {result}'
        }
        
    except Exception as exc:
        logger.error(f"프로필 이미지 업로드 실패 (user_id: {user_id}): {str(exc)}")
        current_task.update_state(
            state='FAILURE',
            meta={'error': str(exc)}
        )
        return {
            'status': 'FAILURE',
            'error': str(exc)
        }

async def _download_and_upload_image_oauth(user_uuid: str, image_url: str) -> str:
    """OAuth 프로필 이미지 다운로드 및 R2 업로드 처리"""
    
    # UUID로 사용자 정보 조회하여 handle 가져오기
    user = db_service.get_user_by_uuid(user_uuid)
    if not user:
        raise ValueError(f"사용자를 찾을 수 없음: {user_uuid}")
    
    user_handle = user.handle
    
    async with httpx.AsyncClient(timeout=15.0) as client:
        # Google에서 이미지 다운로드
        logger.info(f"Google에서 프로필 이미지 다운로드 중 (user_uuid: {user_uuid}, handle: {user_handle})")
        
        response = await client.get(image_url)
        response.raise_for_status()
        
        # Content-Type 확인
        content_type = response.headers.get("content-type", "image/jpeg")
        
        if not is_supported_image_type(content_type):
            raise ValueError(f"지원되지 않는 이미지 타입: {content_type}")
        
        # 이미지 데이터 가져오기
        image_data = response.content
        
        # 크기 검증
        if len(image_data) > MAX_IMAGE_SIZE:
            raise ValueError(f"이미지가 너무 큼: {len(image_data)} bytes (최대: {MAX_IMAGE_SIZE} bytes)")
        
        if len(image_data) == 0:
            raise ValueError("빈 이미지 파일")
        
        # R2 저장 경로 생성 (handle 사용)
        r2_key = f"profiles/{user_handle}/avatar"
        
        # R2에 업로드
        logger.info(f"R2에 업로드 중 (key: {r2_key})")
        public_url = await r2_client.upload_file(r2_key, image_data, content_type)
        
        # 데이터베이스에 프로필 이미지 URL 업데이트 (UUID 사용)
        logger.info(f"데이터베이스에 프로필 이미지 URL 업데이트 중 (user_uuid: {user_uuid})")
        db_success = db_service.update_user_profile_image_by_uuid(user_uuid, public_url)
        
        if db_success:
            logger.info(f"프로필 이미지 업로드 및 DB 업데이트 완료: {public_url}")
        else:
            logger.warning(f"R2 업로드는 성공했지만 DB 업데이트 실패: {public_url}")
            # R2 업로드는 성공했으므로 public_url은 반환
        
        return public_url

async def _download_and_upload_image(user_id: str, image_url: str) -> str:
    """이미지 다운로드 및 R2 업로드 처리"""
    
    async with httpx.AsyncClient(timeout=15.0) as client:
        # Google에서 이미지 다운로드
        logger.info(f"Google에서 프로필 이미지 다운로드 중 (user_id: {user_id})")
        
        response = await client.get(image_url)
        response.raise_for_status()
        
        # Content-Type 확인
        content_type = response.headers.get("content-type", "image/jpeg")
        
        if not is_supported_image_type(content_type):
            raise ValueError(f"지원되지 않는 이미지 타입: {content_type}")
        
        # 이미지 데이터 가져오기
        image_data = response.content
        
        # 크기 검증
        if len(image_data) > MAX_IMAGE_SIZE:
            raise ValueError(f"이미지가 너무 큼: {len(image_data)} bytes (최대: {MAX_IMAGE_SIZE} bytes)")
        
        if len(image_data) == 0:
            raise ValueError("빈 이미지 파일")
        
        # R2 저장 경로 생성
        r2_key = f"profiles/{user_id}/avatar"
        
        # R2에 업로드
        logger.info(f"R2에 업로드 중 (key: {r2_key})")
        public_url = await r2_client.upload_file(r2_key, image_data, content_type)
        
        # 데이터베이스에 프로필 이미지 URL 업데이트
        logger.info(f"데이터베이스에 프로필 이미지 URL 업데이트 중 (user_id: {user_id})")
        db_success = db_service.update_user_profile_image(user_id, public_url)
        
        if db_success:
            logger.info(f"프로필 이미지 업로드 및 DB 업데이트 완료: {public_url}")
        else:
            logger.warning(f"R2 업로드는 성공했지만 DB 업데이트 실패: {public_url}")
            # R2 업로드는 성공했으므로 public_url은 반환
        
        return public_url

@celery_app.task(bind=True, name="upload_user_file")
def upload_user_file_task(self, user_uuid: str, user_handle: str, file_data: bytes, content_type: str, file_type: str):
    """
    사용자가 업로드한 파일을 R2에 업로드하는 Celery 태스크
    
    Args:
        user_id: 사용자 ID
        file_data: 파일 바이너리 데이터
        content_type: 파일 MIME 타입
        file_type: 파일 타입 ("profile" 또는 "banner")
    
    Returns:
        dict: 성공 시 public_url, 실패 시 error 메시지
    """
    try:
        # 태스크 상태 업데이트
        current_task.update_state(
            state='PROGRESS',
            meta={'status': f'{file_type} 이미지 처리 중...'}
        )
        
        # 지원되는 파일 타입 검증
        if not is_supported_image_type(content_type):
            raise ValueError(f"지원되지 않는 이미지 타입: {content_type}")
        
        # 파일 크기 검증
        if len(file_data) > MAX_IMAGE_SIZE:
            raise ValueError(f"이미지가 너무 큼: {len(file_data)} bytes (최대: {MAX_IMAGE_SIZE} bytes)")
        
        if len(file_data) == 0:
            raise ValueError("빈 파일")
        
        # 비동기 함수를 동기적으로 실행
        result = asyncio.run(_upload_user_file(user_uuid, user_handle, file_data, content_type, file_type))
        
        return {
            'status': 'SUCCESS',
            'public_url': result,
            'message': f'{file_type} 이미지가 성공적으로 업로드되었습니다: {result}'
        }
        
    except Exception as exc:
        logger.error(f"{file_type} 이미지 업로드 실패 (user_uuid: {user_uuid}, user_handle: {user_handle}): {str(exc)}")
        current_task.update_state(
            state='FAILURE',
            meta={'error': str(exc)}
        )
        return {
            'status': 'FAILURE',
            'error': str(exc)
        }

async def _upload_user_file(user_uuid: str, user_handle: str, file_data: bytes, content_type: str, file_type: str) -> str:
    """사용자 파일 R2 업로드 처리"""
    
    # R2 저장 경로 생성 (handle 사용)
    if file_type == "profile":
        r2_key = f"profiles/{user_handle}/avatar"
    elif file_type == "banner":
        r2_key = f"profiles/{user_handle}/banner"
    else:
        raise ValueError(f"지원되지 않는 파일 타입: {file_type}")
    
    # R2에 업로드
    logger.info(f"R2에 {file_type} 이미지 업로드 중 (key: {r2_key})")
    public_url = await r2_client.upload_file(r2_key, file_data, content_type)
    
    # 데이터베이스 업데이트 (UUID 사용)
    logger.info(f"데이터베이스에 {file_type} 이미지 URL 업데이트 중 (user_uuid: {user_uuid})")
    if file_type == "profile":
        db_success = db_service.update_user_profile_image_by_uuid(user_uuid, public_url)
    else:  # banner
        db_success = db_service.update_user_banner_image_by_uuid(user_uuid, public_url)
    
    if db_success:
        logger.info(f"{file_type} 이미지 업로드 및 DB 업데이트 완료: {public_url}")
    else:
        logger.warning(f"R2 업로드는 성공했지만 DB 업데이트 실패: {public_url}")
    
    return public_url

@celery_app.task(name="delete_profile_image")
def delete_profile_image_task(user_id: str):
    """
    사용자 프로필 이미지를 R2에서 삭제하고 데이터베이스에서도 제거하는 태스크
    
    Args:
        user_id: 사용자 handle
        
    Returns:
        dict: 결과 상태
    """
    try:
        # R2에서 삭제
        r2_key = f"profiles/{user_id}/avatar"
        r2_result = asyncio.run(r2_client.delete_file(r2_key))
        
        # 데이터베이스에서 프로필 이미지 URL 제거 (NULL로 설정)
        db_result = db_service.update_user_profile_image(user_id, None)
        
        if r2_result and db_result:
            return {
                'status': 'SUCCESS',
                'message': f'프로필 이미지가 R2와 데이터베이스에서 성공적으로 삭제되었습니다 (user_id: {user_id})'
            }
        elif r2_result:
            return {
                'status': 'PARTIAL_SUCCESS',
                'message': f'R2에서는 삭제되었지만 데이터베이스 업데이트에 실패했습니다 (user_id: {user_id})'
            }
        elif db_result:
            return {
                'status': 'PARTIAL_SUCCESS', 
                'message': f'데이터베이스는 업데이트되었지만 R2 삭제에 실패했습니다 (user_id: {user_id})'
            }
        else:
            return {
                'status': 'WARNING',
                'message': '삭제할 프로필 이미지가 없거나 모든 삭제 작업에 실패했습니다'
            }
            
    except Exception as exc:
        logger.error(f"프로필 이미지 삭제 실패 (user_id: {user_id}): {str(exc)}")
        return {
            'status': 'FAILURE',
            'error': str(exc)
        }

