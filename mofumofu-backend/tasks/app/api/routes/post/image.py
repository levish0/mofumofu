from fastapi import APIRouter, HTTPException, UploadFile, File, Form
from pydantic import BaseModel
from app.tasks.post_tasks import upload_post_file_task
import logging

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/image", tags=["image"])


class TaskResponse(BaseModel):
    task_id: str
    status: str
    message: str


@router.post("/upload", response_model=TaskResponse)
async def upload_image(
    user_uuid: str = Form(...),
    filename: str = Form(...),
    file: UploadFile = File(...),
):
    """
    게시글용 이미지를 해시 기반 파일명으로 R2에 업로드하는 태스크를 실행
    """
    try:
        # 파일 데이터 읽기
        file_data = await file.read()

        # Content-Type 가져오기
        content_type = file.content_type or "image/jpeg"

        # 이미지 업로드 태스크 실행 (post_id는 None)
        task = upload_post_file_task.delay(
            user_uuid, None, filename, file_data, content_type, "image", False
        )

        logger.info(
            f"게시글 이미지 업로드 태스크 시작: {task.id} (user_uuid: {user_uuid}, filename: {filename})"
        )

        return TaskResponse(
            task_id=task.id,
            status="PENDING",
            message="게시글 이미지 업로드 태스크가 시작되었습니다",
        )

    except Exception as e:
        logger.error(f"게시글 이미지 업로드 태스크 시작 실패: {str(e)}")
        raise HTTPException(status_code=500, detail=f"태스크 시작 실패: {str(e)}")