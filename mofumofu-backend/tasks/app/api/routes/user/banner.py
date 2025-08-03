from fastapi import APIRouter, HTTPException, UploadFile, File, Form
from pydantic import BaseModel
from app.tasks.profile_tasks import upload_user_file_task
import logging

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/banner", tags=["banner"])

class TaskResponse(BaseModel):
    task_id: str
    status: str
    message: str

@router.post("/upload", response_model=TaskResponse)
async def upload_banner(
    user_uuid: str = Form(...),
    user_handle: str = Form(...),
    file: UploadFile = File(...)
):
    """
    사용자 배너 이미지를 R2에 업로드하는 태스크를 실행
    """
    try:
        # 파일 데이터 읽기
        file_data = await file.read()
        
        # Content-Type 가져오기
        content_type = file.content_type or "image/jpeg"
        
        # Celery 태스크 실행
        task = upload_user_file_task.delay(user_uuid, user_handle, file_data, content_type, "banner")
        
        logger.info(f"배너 이미지 업로드 태스크 시작: {task.id} (user_uuid: {user_uuid}, user_handle: {user_handle}, filename: {file.filename})")
        
        return TaskResponse(
            task_id=task.id,
            status="PENDING",
            message="배너 이미지 업로드 태스크가 시작되었습니다"
        )
        
    except Exception as e:
        logger.error(f"배너 이미지 업로드 태스크 시작 실패: {str(e)}")
        raise HTTPException(status_code=500, detail=f"태스크 시작 실패: {str(e)}")