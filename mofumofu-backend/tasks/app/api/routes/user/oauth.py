from fastapi import APIRouter, HTTPException
from pydantic import BaseModel
from app.tasks.profile_tasks import upload_profile_image_task, delete_profile_image_task
import logging

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/oauth", tags=["oauth"])

class ProfileImageOauthUploadRequest(BaseModel):
    user_id: str
    image_url: str

class ProfileImageDeleteRequest(BaseModel):
    user_id: str

class TaskResponse(BaseModel):
    task_id: str
    status: str
    message: str

@router.post("/upload-avatar", response_model=TaskResponse)
async def upload_oauth_profile_image(request: ProfileImageOauthUploadRequest):
    """
    Google OAuth 프로필 이미지를 R2에 업로드하는 태스크를 실행
    """
    try:
        # Celery 태스크 실행
        task = upload_profile_image_task.delay(request.user_id, request.image_url)
        
        logger.info(f"OAuth 프로필 이미지 업로드 태스크 시작: {task.id} (user_id: {request.user_id})")
        
        return TaskResponse(
            task_id=task.id,
            status="PENDING",
            message="OAuth 프로필 이미지 업로드 태스크가 시작되었습니다"
        )
        
    except Exception as e:
        logger.error(f"OAuth 프로필 이미지 업로드 태스크 시작 실패: {str(e)}")
        raise HTTPException(status_code=500, detail=f"태스크 시작 실패: {str(e)}")

@router.delete("/delete-avatar", response_model=TaskResponse)
async def delete_oauth_profile_image(request: ProfileImageDeleteRequest):
    """
    OAuth 프로필 이미지를 R2에서 삭제하는 태스크를 실행
    """
    try:
        # Celery 태스크 실행
        task = delete_profile_image_task.delay(request.user_id)
        
        logger.info(f"OAuth 프로필 이미지 삭제 태스크 시작: {task.id} (user_id: {request.user_id})")
        
        return TaskResponse(
            task_id=task.id,
            status="PENDING",
            message="OAuth 프로필 이미지 삭제 태스크가 시작되었습니다"
        )
        
    except Exception as e:
        logger.error(f"OAuth 프로필 이미지 삭제 태스크 시작 실패: {str(e)}")
        raise HTTPException(status_code=500, detail=f"태스크 시작 실패: {str(e)}")