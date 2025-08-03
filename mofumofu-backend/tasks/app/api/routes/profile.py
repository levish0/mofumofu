from fastapi import APIRouter, HTTPException, BackgroundTasks
from pydantic import BaseModel
from celery.result import AsyncResult
from app.tasks.profile_tasks import upload_profile_image_task, delete_profile_image_task
from app.core.celery_app import celery_app
import logging

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/profile", tags=["profile"])

class ProfileImageUploadRequest(BaseModel):
    user_id: str
    image_url: str

class ProfileImageDeleteRequest(BaseModel):
    user_id: str

class TaskResponse(BaseModel):
    task_id: str
    status: str
    message: str

class TaskStatusResponse(BaseModel):
    task_id: str
    status: str
    result: dict = None
    error: str = None

@router.post("/upload-image", response_model=TaskResponse)
async def upload_profile_image(request: ProfileImageUploadRequest):
    """
    Google OAuth 프로필 이미지를 R2에 업로드하는 태스크를 실행
    """
    try:
        # Celery 태스크 실행
        task = upload_profile_image_task.delay(request.user_id, request.image_url)
        
        logger.info(f"프로필 이미지 업로드 태스크 시작: {task.id} (user_id: {request.user_id})")
        
        return TaskResponse(
            task_id=task.id,
            status="PENDING",
            message="프로필 이미지 업로드 태스크가 시작되었습니다"
        )
        
    except Exception as e:
        logger.error(f"프로필 이미지 업로드 태스크 시작 실패: {str(e)}")
        raise HTTPException(status_code=500, detail=f"태스크 시작 실패: {str(e)}")

@router.delete("/delete-image", response_model=TaskResponse)
async def delete_profile_image(request: ProfileImageDeleteRequest):
    """
    사용자 프로필 이미지를 R2에서 삭제하는 태스크를 실행
    """
    try:
        # Celery 태스크 실행
        task = delete_profile_image_task.delay(request.user_id)
        
        logger.info(f"프로필 이미지 삭제 태스크 시작: {task.id} (user_id: {request.user_id})")
        
        return TaskResponse(
            task_id=task.id,
            status="PENDING",
            message="프로필 이미지 삭제 태스크가 시작되었습니다"
        )
        
    except Exception as e:
        logger.error(f"프로필 이미지 삭제 태스크 시작 실패: {str(e)}")
        raise HTTPException(status_code=500, detail=f"태스크 시작 실패: {str(e)}")

@router.get("/task-status/{task_id}", response_model=TaskStatusResponse)
async def get_task_status(task_id: str):
    """
    태스크 실행 상태와 결과를 조회
    """
    try:
        # Celery 태스크 결과 조회
        result = AsyncResult(task_id, app=celery_app)
        
        response = TaskStatusResponse(
            task_id=task_id,
            status=result.status
        )
        
        if result.status == 'PENDING':
            response.result = {"message": "태스크가 대기 중입니다"}
        elif result.status == 'PROGRESS':
            response.result = result.result
        elif result.status == 'SUCCESS':
            response.result = result.result
        elif result.status == 'FAILURE':
            response.error = str(result.result)
        
        return response
        
    except Exception as e:
        logger.error(f"태스크 상태 조회 실패: {str(e)}")
        raise HTTPException(status_code=500, detail=f"태스크 상태 조회 실패: {str(e)}")

@router.get("/health")
async def health_check():
    """
    프로필 서비스 헬스 체크
    """
    return {"status": "healthy", "service": "profile-tasks"}