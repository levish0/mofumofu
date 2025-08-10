import httpx
import asyncio
from celery import current_task
from app.core.celery_app import celery_app
from app.services.r2_client import r2_client
from app.services.db_service import db_service
from app.utils import (
    ValidationError,
    create_success_response,
    create_failure_response,
    validate_image_type,
    validate_file_size,
    validate_uuid,
)
from app.core.logging_config import (
    get_task_logger,
    log_task_start,
    log_task_success,
    log_task_error,
)
import time
from typing import Dict, Any

logger = get_task_logger(__name__)

# 지원되는 이미지 타입
SUPPORTED_IMAGE_TYPES = {
    "image/jpeg",
    "image/jpg",
    "image/png",
    "image/webp",
    "image/gif",
}

# 최대 이미지 크기 (8MB)
MAX_IMAGE_SIZE = 8 * 1024 * 1024


def is_supported_image_type(content_type: str) -> bool:
    """지원되는 이미지 타입인지 확인"""
    return content_type.lower() in SUPPORTED_IMAGE_TYPES


@celery_app.task(bind=True, name="upload_oauth_profile_image")
def upload_oauth_profile_image_task(
    self, user_id: str, image_url: str
) -> Dict[str, Any]:
    """
    Google OAuth에서 받은 프로필 이미지를 R2에 업로드하는 Celery 태스크

    Args:
        user_id: 사용자 ID
        image_url: Google 프로필 이미지 URL

    Returns:
        dict: 성공 시 public_url, 실패 시 error 메시지
    """
    start_time = time.time()
    task_logger = get_task_logger(__name__, task_id=self.request.id, user_id=user_id)

    try:
        # 입력값 검증
        validate_uuid(user_id)
        if not image_url or not image_url.startswith(("http://", "https://")):
            raise ValidationError(f"잘못된 이미지 URL: {image_url}")

        log_task_start(task_logger, "upload_oauth_profile_image", user_id=user_id)

        # 태스크 상태 업데이트
        current_task.update_state(
            state="PROGRESS", meta={"status": "Google에서 이미지 다운로드 중..."}
        )

        # 비동기 함수를 동기적으로 실행
        result = asyncio.run(_download_and_upload_oauth_image(user_id, image_url))

        duration = time.time() - start_time
        log_task_success(
            task_logger, "upload_oauth_profile_image", duration, user_id=user_id
        )

        return create_success_response(
            f"프로필 이미지가 성공적으로 업로드되었습니다: {result}", public_url=result
        )

    except (ValidationError, ValueError) as exc:
        duration = time.time() - start_time
        log_task_error(
            task_logger,
            "upload_oauth_profile_image",
            str(exc),
            duration,
            user_id=user_id,
        )
        current_task.update_state(state="FAILURE", meta={"error": str(exc)})
        return create_failure_response(str(exc))

    except Exception as exc:
        duration = time.time() - start_time
        log_task_error(
            task_logger,
            "upload_oauth_profile_image",
            str(exc),
            duration,
            user_id=user_id,
        )
        current_task.update_state(state="FAILURE", meta={"error": str(exc)})
        return create_failure_response(str(exc))


async def _download_and_upload_oauth_image(user_uuid: str, image_url: str) -> str:
    """OAuth 프로필 이미지 다운로드 및 R2 업로드 처리"""

    # UUID로 사용자 정보 조회하여 handle 가져오기
    user = db_service.get_user_by_uuid(user_uuid)
    if not user:
        raise ValueError(f"사용자를 찾을 수 없음: {user_uuid}")

    user_handle = user.handle

    async with httpx.AsyncClient(timeout=15.0) as client:
        # Google에서 이미지 다운로드
        logger.info(
            f"Google에서 프로필 이미지 다운로드 중 (user_uuid: {user_uuid}, handle: {user_handle})"
        )

        response = await client.get(image_url)
        response.raise_for_status()

        # Content-Type 확인 및 검증
        content_type = response.headers.get("content-type", "image/jpeg")
        validate_image_type(content_type, SUPPORTED_IMAGE_TYPES)

        # 이미지 데이터 가져오기 및 검증
        image_data = response.content
        validate_file_size(image_data, MAX_IMAGE_SIZE)

        # R2 저장 경로 생성 (handle 사용)
        r2_key = f"profiles/{user_handle}/avatar"

        # R2에 업로드
        logger.info(f"R2에 업로드 중 (key: {r2_key})")
        public_url = await r2_client.upload_file(r2_key, image_data, content_type)

        # 데이터베이스에 프로필 이미지 URL 업데이트 (UUID 사용)
        logger.info(
            f"데이터베이스에 프로필 이미지 URL 업데이트 중 (user_uuid: {user_uuid})"
        )
        db_success = db_service.update_user_profile_image_by_uuid(user_uuid, public_url)

        if db_success:
            logger.info(f"프로필 이미지 업로드 및 DB 업데이트 완료: {public_url}")
        else:
            logger.warning(f"R2 업로드는 성공했지만 DB 업데이트 실패: {public_url}")
            # R2 업로드는 성공했으므로 public_url은 반환

        return public_url


@celery_app.task(bind=True, name="upload_user_file")
def upload_user_file_task(
    self,
    user_uuid: str,
    user_handle: str,
    filename: str,
    file_data: bytes,
    content_type: str,
    file_type: str,
    replace: bool = False,
):
    # 디버깅용 로그 추가
    """
    사용자가 업로드한 파일을 R2에 업로드하는 Celery 태스크

    Args:
        user_uuid: 사용자 UUID
        user_handle: 사용자 handle
        filename: 업로드할 파일명
        file_data: 파일 바이너리 데이터
        content_type: 파일 MIME 타입
        file_type: 파일 타입 ("profile" 또는 "banner")
        replace: 기존 파일 교체 여부 (기본값: False)

    Returns:
        dict: 성공 시 public_url, 실패 시 error 메시지
    """
    try:
        # 태스크 상태 업데이트
        current_task.update_state(
            state="PROGRESS", meta={"status": f"{file_type} 이미지 처리 중..."}
        )

        # 파일 타입 및 크기 검증
        validate_image_type(content_type, SUPPORTED_IMAGE_TYPES)
        validate_file_size(file_data, MAX_IMAGE_SIZE)

        # replace=True인 경우 기존 파일 먼저 삭제
        if replace:
            current_task.update_state(
                state="PROGRESS", meta={"status": f"기존 {file_type} 이미지 삭제 중..."}
            )
            asyncio.run(_delete_user_file(user_uuid, user_handle, file_type))

        # 비동기 함수를 동기적으로 실행
        result = asyncio.run(
            _upload_user_file(
                user_uuid, user_handle, filename, file_data, content_type, file_type
            )
        )

        return {
            "status": "SUCCESS",
            "public_url": result,
            "message": f"{file_type} 이미지가 성공적으로 업로드되었습니다: {result}",
        }

    except Exception as exc:
        logger.error(
            f"{file_type} 이미지 업로드 실패 (user_uuid: {user_uuid}, user_handle: {user_handle}, filename: {filename}): {str(exc)}"
        )
        current_task.update_state(state="FAILURE", meta={"error": str(exc)})
        return {"status": "FAILURE", "error": str(exc)}


@celery_app.task(bind=True, name="delete_user_file")
def delete_user_file_task(self, user_uuid: str, user_handle: str, file_type: str):
    """
    사용자 파일을 R2에서 삭제하고 데이터베이스에서도 제거하는 태스크

    Args:
        user_uuid: 사용자 UUID
        user_handle: 사용자 handle
        file_type: 파일 타입 ("profile" 또는 "banner")

    Returns:
        dict: 결과 상태
    """
    try:
        # 태스크 상태 업데이트
        current_task.update_state(
            state="PROGRESS", meta={"status": f"{file_type} 이미지 삭제 중..."}
        )

        # 비동기 함수를 동기적으로 실행
        asyncio.run(_delete_user_file(user_uuid, user_handle, file_type))

        return {
            "status": "SUCCESS",
            "message": f"{file_type} 이미지가 성공적으로 삭제되었습니다",
        }

    except Exception as exc:
        logger.error(
            f"{file_type} 이미지 삭제 실패 (user_uuid: {user_uuid}, user_handle: {user_handle}): {str(exc)}"
        )
        current_task.update_state(state="FAILURE", meta={"error": str(exc)})
        return {"status": "FAILURE", "error": str(exc)}


async def _delete_user_file(user_uuid: str, user_handle: str, file_type: str) -> bool:
    """사용자 파일 R2에서 삭제 및 DB 업데이트 처리"""

    # DB에서 현재 이미지 URL 확인
    user = db_service.get_user_by_uuid(user_uuid)
    if not user:
        raise ValueError(f"사용자를 찾을 수 없음: {user_uuid}")

    # 현재 이미지 URL에서 파일명 추출
    if file_type == "profile":
        current_url = user.profile_image
    elif file_type == "banner":
        current_url = user.banner_image
    else:
        raise ValueError(f"지원되지 않는 파일 타입: {file_type}")

    if not current_url:
        logger.info(f"삭제할 {file_type} 이미지가 없음 (user_uuid: {user_uuid})")
        return True  # 이미지가 없으면 삭제 성공으로 처리

    # URL에서 파일명 추출 (예: https://domain/profiles/handle/avatar/filename.jpg -> filename.jpg)
    try:
        filename = current_url.split("/")[-1]
        if file_type == "profile":
            r2_key = f"profiles/{user_handle}/avatar/{filename}"
        else:  # banner
            r2_key = f"profiles/{user_handle}/banner/{filename}"
    except Exception as e:
        logger.error(f"URL에서 파일명 추출 실패: {current_url}, error: {str(e)}")
        # 기본 경로로 시도
        if file_type == "profile":
            r2_key = f"profiles/{user_handle}/avatar"
        else:
            r2_key = f"profiles/{user_handle}/banner"

    # R2에서 삭제
    logger.info(f"R2에서 {file_type} 이미지 삭제 중 (key: {r2_key})")
    r2_result = await r2_client.delete_file(r2_key)

    # 데이터베이스에서 이미지 URL 제거 (NULL로 설정)
    logger.info(
        f"데이터베이스에서 {file_type} 이미지 URL 제거 중 (user_uuid: {user_uuid})"
    )
    if file_type == "profile":
        db_result = db_service.update_user_profile_image_by_uuid(user_uuid, None)
    else:  # banner
        db_result = db_service.update_user_banner_image_by_uuid(user_uuid, None)

    if r2_result and db_result:
        logger.info(f"{file_type} 이미지 R2 및 DB 삭제 완료")
        return True
    elif r2_result:
        logger.warning("R2에서는 삭제되었지만 DB 업데이트 실패")
        # R2에서는 삭제되었으므로 부분적 성공으로 처리
        return True
    elif db_result:
        logger.warning("DB는 업데이트되었지만 R2 삭제 실패")
        return True
    else:
        logger.warning(f"삭제할 {file_type} 이미지가 없거나 모든 삭제 작업 실패")
        return True  # 파일이 없는 경우도 삭제 성공으로 처리


async def _upload_user_file(
    user_uuid: str,
    user_handle: str,
    filename: str,
    file_data: bytes,
    content_type: str,
    file_type: str,
) -> str:
    """사용자 파일 R2 업로드 처리"""

    # R2 저장 경로 생성 (handle과 filename 사용)
    if file_type == "profile":
        r2_key = f"profiles/{user_handle}/avatar/{filename}"
    elif file_type == "banner":
        r2_key = f"profiles/{user_handle}/banner/{filename}"
    else:
        raise ValueError(f"지원되지 않는 파일 타입: {file_type}")

    # R2에 업로드
    logger.info(f"R2에 {file_type} 이미지 업로드 중 (key: {r2_key})")
    public_url = await r2_client.upload_file(r2_key, file_data, content_type)

    # 데이터베이스 업데이트 (UUID 사용)
    logger.info(
        f"데이터베이스에 {file_type} 이미지 URL 업데이트 중 (user_uuid: {user_uuid})"
    )
    if file_type == "profile":
        db_success = db_service.update_user_profile_image_by_uuid(user_uuid, public_url)
    else:  # banner
        db_success = db_service.update_user_banner_image_by_uuid(user_uuid, public_url)

    if db_success:
        logger.info(f"{file_type} 이미지 업로드 및 DB 업데이트 완료: {public_url}")
    else:
        logger.warning(f"R2 업로드는 성공했지만 DB 업데이트 실패: {public_url}")

    return public_url


@celery_app.task(name="delete_oauth_profile_image")
def delete_oauth_profile_image_task(user_id: str):
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
        r2_result = r2_client.delete_file_sync(r2_key)

        # 데이터베이스에서 프로필 이미지 URL 제거 (NULL로 설정)
        db_result = db_service.update_user_profile_image(user_id, None)

        if r2_result and db_result:
            return {
                "status": "SUCCESS",
                "message": f"프로필 이미지가 R2와 데이터베이스에서 성공적으로 삭제되었습니다 (user_id: {user_id})",
            }
        elif r2_result:
            return {
                "status": "PARTIAL_SUCCESS",
                "message": f"R2에서는 삭제되었지만 데이터베이스 업데이트에 실패했습니다 (user_id: {user_id})",
            }
        elif db_result:
            return {
                "status": "PARTIAL_SUCCESS",
                "message": f"데이터베이스는 업데이트되었지만 R2 삭제에 실패했습니다 (user_id: {user_id})",
            }
        else:
            return {
                "status": "WARNING",
                "message": "삭제할 프로필 이미지가 없거나 모든 삭제 작업에 실패했습니다",
            }

    except Exception as exc:
        logger.error(f"프로필 이미지 삭제 실패 (user_id: {user_id}): {str(exc)}")
        return {"status": "FAILURE", "error": str(exc)}
