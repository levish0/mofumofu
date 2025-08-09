from fastapi import APIRouter, HTTPException, UploadFile, File, Form
from pydantic import BaseModel
from app.tasks.post_tasks import upload_post_file_task, delete_post_file_task
import logging

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/thumbnail", tags=["thumbnail"])

class TaskResponse(BaseModel):
    task_id: str
    status: str
    message: str

@router.post("/upload", response_model=TaskResponse)
async def upload_thumbnail(
    user_uuid: str = Form(...),
    post_id: str = Form(...),
    filename: str = Form(...),
    file: UploadFile = File(...)
):
    """
    포스트 썸네일 이미지를 R2에 업로드하는 태스크를 실행
    """
    try:
        # 파일 데이터 읽기
        file_data = await file.read()
        
        # Content-Type 가져오기
        content_type = file.content_type or "image/jpeg"
        
        # Celery 태스크 실행
        task = upload_post_file_task.delay(user_uuid, post_id, filename, file_data, content_type, "thumbnail", False)
        
        logger.info(f"포스트 썸네일 업로드 태스크 시작: {task.id} (user_uuid: {user_uuid}, post_id: {post_id}, filename: {file.filename})")
        
        return TaskResponse(
            task_id=task.id,
            status="PENDING",
            message="포스트 썸네일 업로드 태스크가 시작되었습니다"
        )
        
    except Exception as e:
        logger.error(f"포스트 썸네일 업로드 태스크 시작 실패: {str(e)}")
        raise HTTPException(status_code=500, detail=f"태스크 시작 실패: {str(e)}")

@router.delete("/delete", response_model=TaskResponse)
async def delete_thumbnail(
    user_uuid: str = Form(...),
    post_id: str = Form(...)
):
    """
    포스트 썸네일 이미지를 R2에서 삭제하는 태스크를 실행
    """
    try:
        # Celery 태스크 실행
        task = delete_post_file_task.delay(user_uuid, post_id, "thumbnail")
        
        logger.info(f"포스트 썸네일 삭제 태스크 시작: {task.id} (user_uuid: {user_uuid}, post_id: {post_id})")
        
        return TaskResponse(
            task_id=task.id,
            status="PENDING",
            message="포스트 썸네일 삭제 태스크가 시작되었습니다"
        )
        
    except Exception as e:
        logger.error(f"포스트 썸네일 삭제 태스크 시작 실패: {str(e)}")
        raise HTTPException(status_code=500, detail=f"태스크 시작 실패: {str(e)}")

@router.put("/update", response_model=TaskResponse)
async def update_thumbnail(
    user_uuid: str = Form(...),
    post_id: str = Form(...),
    filename: str = Form(...),
    file: UploadFile = File(...)
):
    """
    포스트 썸네일 이미지를 업데이트 (기존 삭제 후 새 이미지 업로드)
    """
    try:
        # 파일 데이터 읽기
        file_data = await file.read()
        
        # Content-Type 가져오기
        content_type = file.content_type or "image/jpeg"
        
        # 기존 이미지 삭제 후 새 이미지 업로드 태스크 실행
        task = upload_post_file_task.delay(user_uuid, post_id, filename, file_data, content_type, "thumbnail", True)
        
        logger.info(f"포스트 썸네일 업데이트 태스크 시작: {task.id} (user_uuid: {user_uuid}, post_id: {post_id}, filename: {file.filename})")
        
        return TaskResponse(
            task_id=task.id,
            status="PENDING",
            message="포스트 썸네일 업데이트 태스크가 시작되었습니다"
        )
        
    except Exception as e:
        logger.error(f"포스트 썸네일 업데이트 태스크 시작 실패: {str(e)}")
        raise HTTPException(status_code=500, detail=f"태스크 시작 실패: {str(e)}")