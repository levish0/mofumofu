import asyncio
from celery import current_task
from app.core.celery_app import celery_app
from app.services.r2_client import r2_client
from app.services.db_service import db_service
import logging

logger = logging.getLogger(__name__)

@celery_app.task(bind=True, name="upload_post_thumbnail")
def upload_post_thumbnail_task(self, user_uuid: str, post_id: str, filename: str, file_data: bytes, content_type: str):
    """
    포스트 썸네일 이미지를 R2에 업로드하고 데이터베이스에 URL 저장
    """
    try:
        # 태스크 상태 업데이트
        current_task.update_state(
            state='PROGRESS',
            meta={'status': '포스트 썸네일 업로드 중...'}
        )
        
        # 비동기 함수를 동기적으로 실행
        result = asyncio.run(_upload_post_thumbnail(user_uuid, post_id, filename, file_data, content_type))
        
        return {
            'status': 'SUCCESS',
            'public_url': result,
            'post_id': post_id,
            'message': f'포스트 썸네일이 성공적으로 업로드되었습니다: {result}'
        }
        
    except Exception as exc:
        logger.error(f"포스트 썸네일 업로드 실패 (post_id: {post_id}): {str(exc)}")
        current_task.update_state(
            state='FAILURE',
            meta={'error': str(exc)}
        )
        return {
            'status': 'FAILURE',
            'error': str(exc),
            'post_id': post_id
        }

async def _upload_post_thumbnail(user_uuid: str, post_id: str, filename: str, file_data: bytes, content_type: str) -> str:
    """포스트 썸네일 R2 업로드 처리"""
    
    # 파일 확장자 결정
    if content_type == "image/png":
        extension = ".png"
    elif content_type == "image/gif":
        extension = ".gif"
    elif content_type == "image/webp":
        extension = ".webp"
    else:
        extension = ".jpg"
    
    # R2 저장 경로 생성
    r2_key = f"posts/{post_id}/thumbnail_{filename}{extension}"
    
    # R2에 업로드
    logger.info(f"R2에 포스트 썸네일 업로드 중 (key: {r2_key})")
    public_url = await r2_client.upload_file(r2_key, file_data, content_type)
    
    # 데이터베이스에 썸네일 URL 업데이트
    logger.info(f"데이터베이스에 썸네일 URL 업데이트 중 (post_id: {post_id})")
    db_success = db_service.update_post_thumbnail(post_id, public_url)
    
    if db_success:
        logger.info(f"포스트 썸네일 업로드 및 DB 업데이트 완료: {public_url}")
    else:
        logger.warning(f"R2 업로드는 성공했지만 DB 업데이트 실패: {public_url}")
        # R2에서 업로드된 파일 삭제
        await r2_client.delete_file(r2_key)
        raise Exception("데이터베이스 업데이트 실패")
    
    return public_url

@celery_app.task(bind=True, name="upload_post_file")
def upload_post_file_task(self, user_uuid: str, post_id: str, filename: str, file_data: bytes, content_type: str, file_type: str, replace: bool = False):
    """
    포스트 파일을 R2에 업로드하는 Celery 태스크
    
    Args:
        user_uuid: 사용자 UUID
        post_id: 포스트 ID
        filename: 업로드할 파일명
        file_data: 파일 바이너리 데이터
        content_type: 파일 MIME 타입
        file_type: 파일 타입 ("thumbnail" 또는 "attachment")
        replace: 기존 파일 교체 여부 (기본값: False)
    
    Returns:
        dict: 성공 시 public_url, 실패 시 error 메시지
    """
    try:
        # 태스크 상태 업데이트
        current_task.update_state(
            state='PROGRESS',
            meta={'status': f'{file_type} 파일 처리 중...'}
        )
        
        # replace=True인 경우 기존 파일 먼저 삭제
        if replace:
            current_task.update_state(
                state='PROGRESS',
                meta={'status': f'기존 {file_type} 파일 삭제 중...'}
            )
            asyncio.run(_delete_post_file(user_uuid, post_id, file_type))
        
        # 비동기 함수를 동기적으로 실행
        result = asyncio.run(_upload_post_file(user_uuid, post_id, filename, file_data, content_type, file_type))
        
        return {
            'status': 'SUCCESS',
            'public_url': result,
            'post_id': post_id,
            'message': f'{file_type} 파일이 성공적으로 업로드되었습니다: {result}'
        }
        
    except Exception as exc:
        logger.error(f"{file_type} 파일 업로드 실패 (post_id: {post_id}, filename: {filename}): {str(exc)}")
        current_task.update_state(
            state='FAILURE',
            meta={'error': str(exc)}
        )
        return {
            'status': 'FAILURE',
            'error': str(exc),
            'post_id': post_id
        }

@celery_app.task(bind=True, name="delete_post_file")
def delete_post_file_task(self, user_uuid: str, post_id: str, file_type: str):
    """
    포스트 파일을 R2에서 삭제하고 데이터베이스에서도 제거하는 태스크
    
    Args:
        user_uuid: 사용자 UUID
        post_id: 포스트 ID
        file_type: 파일 타입 ("thumbnail" 또는 "attachment")
        
    Returns:
        dict: 결과 상태
    """
    try:
        # 태스크 상태 업데이트
        current_task.update_state(
            state='PROGRESS',
            meta={'status': f'{file_type} 파일 삭제 중...'}
        )
        
        # 비동기 함수를 동기적으로 실행
        result = asyncio.run(_delete_post_file(user_uuid, post_id, file_type))
        
        return {
            'status': 'SUCCESS',
            'message': f'{file_type} 파일이 성공적으로 삭제되었습니다',
            'post_id': post_id
        }
        
    except Exception as exc:
        logger.error(f"{file_type} 파일 삭제 실패 (post_id: {post_id}): {str(exc)}")
        current_task.update_state(
            state='FAILURE',
            meta={'error': str(exc)}
        )
        return {
            'status': 'FAILURE',
            'error': str(exc),
            'post_id': post_id
        }

async def _delete_post_file(user_uuid: str, post_id: str, file_type: str) -> bool:
    """포스트 파일 R2에서 삭제 및 DB 업데이트 처리"""
    
    # DB에서 포스트 정보 확인
    post = db_service.get_post_by_id(post_id)
    if not post:
        raise ValueError(f"포스트를 찾을 수 없음: {post_id}")
    
    # 현재 파일 URL에서 파일명 추출
    if file_type == "thumbnail":
        current_url = post.thumbnail_image
    else:
        raise ValueError(f"지원되지 않는 파일 타입: {file_type}")
    
    if not current_url:
        logger.info(f"삭제할 {file_type} 파일이 없음 (post_id: {post_id})")
        return True
    
    # URL에서 파일명과 경로 추출
    try:
        # posts/{post_id}/thumbnail_filename.ext 형태에서 전체 경로 추출
        url_parts = current_url.split('/')
        post_id_index = url_parts.index('posts') + 1
        if url_parts[post_id_index] == post_id:
            filename = url_parts[-1]
            r2_key = f"posts/{post_id}/{filename}"
        else:
            raise ValueError("URL 구조가 예상과 다름")
    except Exception as e:
        logger.error(f"URL에서 파일 경로 추출 실패: {current_url}, error: {str(e)}")
        # 기본 경로로 시도
        r2_key = f"posts/{post_id}/thumbnail"
    
    # R2에서 삭제
    logger.info(f"R2에서 {file_type} 파일 삭제 중 (key: {r2_key})")
    r2_result = await r2_client.delete_file(r2_key)
    
    # 데이터베이스에서 파일 URL 제거 (NULL로 설정)
    logger.info(f"데이터베이스에서 {file_type} URL 제거 중 (post_id: {post_id})")
    if file_type == "thumbnail":
        db_result = db_service.update_post_thumbnail(post_id, None)
    else:
        db_result = False
    
    if r2_result and db_result:
        logger.info(f"{file_type} 파일 R2 및 DB 삭제 완료")
        return True
    elif r2_result:
        logger.warning("R2에서는 삭제되었지만 DB 업데이트 실패")
        return True
    elif db_result:
        logger.warning("DB는 업데이트되었지만 R2 삭제 실패")
        return True
    else:
        logger.warning(f"삭제할 {file_type} 파일이 없거나 모든 삭제 작업 실패")
        return True

async def _upload_post_file(user_uuid: str, post_id: str, filename: str, file_data: bytes, content_type: str, file_type: str) -> str:
    """포스트 파일 R2 업로드 처리"""
    
    # 파일 확장자 결정
    if content_type == "image/png":
        extension = ".png"
    elif content_type == "image/gif":
        extension = ".gif"
    elif content_type == "image/webp":
        extension = ".webp"
    else:
        extension = ".jpg"
    
    # R2 저장 경로 생성
    if file_type == "thumbnail":
        r2_key = f"posts/{post_id}/thumbnail_{filename}{extension}"
    elif file_type == "attachment":
        r2_key = f"posts/{post_id}/attachment_{filename}{extension}"
    else:
        raise ValueError(f"지원되지 않는 파일 타입: {file_type}")
    
    # R2에 업로드
    logger.info(f"R2에 {file_type} 파일 업로드 중 (key: {r2_key})")
    public_url = await r2_client.upload_file(r2_key, file_data, content_type)
    
    # 데이터베이스 업데이트
    logger.info(f"데이터베이스에 {file_type} URL 업데이트 중 (post_id: {post_id})")
    if file_type == "thumbnail":
        db_success = db_service.update_post_thumbnail(post_id, public_url)
    else:
        # attachment의 경우 별도 처리 로직 필요
        db_success = True
    
    if db_success:
        logger.info(f"{file_type} 파일 업로드 및 DB 업데이트 완료: {public_url}")
    else:
        logger.warning(f"R2 업로드는 성공했지만 DB 업데이트 실패: {public_url}")
        # R2에서 업로드된 파일 삭제
        await r2_client.delete_file(r2_key)
        raise Exception("데이터베이스 업데이트 실패")
    
    return public_url