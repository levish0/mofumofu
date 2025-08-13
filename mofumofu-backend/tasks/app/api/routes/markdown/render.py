from fastapi import APIRouter, HTTPException
from pydantic import BaseModel
from typing import Optional, Dict, Any, List
from app.tasks.markdown_tasks import (
    render_and_cache_markdown_task,
    get_cached_markdown_task,
    invalidate_markdown_cache_task,
    warm_up_markdown_cache_task
)
import logging

logger = logging.getLogger(__name__)
router = APIRouter()


class MarkdownRenderRequest(BaseModel):
    post_id: str
    content: str
    cache_ttl: Optional[int] = 86400  # 기본 24시간 (글 내용은 자주 변경되지 않음)


class MarkdownCacheRequest(BaseModel):
    post_id: str


class MarkdownWarmupRequest(BaseModel):
    posts: List[dict]  # [{"post_id": "uuid", "content": "markdown"}]
    cache_ttl: Optional[int] = 86400  # 기본 24시간


class TaskStatusResponse(BaseModel):
    task_id: str
    status: str
    result: Optional[Dict[str, Any]] = None
    meta: Optional[Dict[str, Any]] = None


@router.post("/render", summary="마크다운 렌더링 및 캐시", tags=["markdown"])
async def render_markdown(request: MarkdownRenderRequest) -> Dict[str, Any]:
    """
    마크다운을 렌더링하고 Redis에 캐시합니다.
    캐시가 있으면 캐시된 결과를 즉시 반환합니다.
    """
    try:
        # 먼저 캐시 확인 (동기적)
        cached_result = get_cached_markdown_task.apply(args=[request.post_id])
        
        if cached_result.result["status"] == "SUCCESS":
            logger.info(f"캐시된 마크다운 결과 반환: post_id={request.post_id}")
            return {
                "success": True,
                "data": cached_result.result["data"],
                "cached": True,
                "cache_key": cached_result.result.get("cache_key")
            }
        
        # 캐시가 없으면 렌더링 (비동기 태스크)
        task = render_and_cache_markdown_task.delay(request.post_id, request.content, request.cache_ttl)
        
        # 태스크 완료까지 대기 (타임아웃 30초)
        result = task.get(timeout=30)
        
        if result["status"] == "SUCCESS":
            return {
                "success": True,
                "data": result["data"],
                "cached": result.get("cached", False),
                "cache_key": result.get("cache_key")
            }
        else:
            raise HTTPException(
                status_code=500,
                detail=f"마크다운 렌더링 실패: {result.get('error', '알 수 없는 오류')}"
            )
            
    except Exception as e:
        logger.error(f"마크다운 렌더링 API 오류: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))


@router.post("/render-async", summary="마크다운 비동기 렌더링", tags=["markdown"])
async def render_markdown_async(request: MarkdownRenderRequest) -> Dict[str, str]:
    """
    마크다운 렌더링을 비동기로 요청하고 태스크 ID를 반환합니다.
    결과는 별도로 조회해야 합니다.
    """
    try:
        task = render_and_cache_markdown_task.delay(request.post_id, request.content, request.cache_ttl)
        
        return {
            "success": True,
            "task_id": task.id,
            "message": "마크다운 렌더링 태스크가 시작되었습니다"
        }
        
    except Exception as e:
        logger.error(f"마크다운 비동기 렌더링 API 오류: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))


@router.post("/get-cached", summary="캐시된 마크다운 조회", tags=["markdown"])
async def get_cached_markdown(request: MarkdownCacheRequest) -> Dict[str, Any]:
    """
    캐시된 마크다운 렌더링 결과를 조회합니다.
    """
    try:
        result = get_cached_markdown_task.apply(args=[request.post_id])
        
        if result.result["status"] == "SUCCESS":
            return {
                "success": True,
                "data": result.result["data"],
                "cache_key": result.result.get("cache_key")
            }
        elif result.result["status"] == "NOT_FOUND":
            return {
                "success": False,
                "message": "캐시된 결과가 없습니다",
                "cache_key": result.result.get("cache_key")
            }
        else:
            raise HTTPException(
                status_code=500,
                detail=f"캐시 조회 실패: {result.result.get('error', '알 수 없는 오류')}"
            )
            
    except Exception as e:
        logger.error(f"캐시 조회 API 오류: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))


@router.post("/invalidate-cache", summary="마크다운 캐시 무효화", tags=["markdown"])
async def invalidate_markdown_cache(request: MarkdownCacheRequest) -> Dict[str, Any]:
    """
    특정 마크다운 콘텐츠의 캐시를 무효화합니다.
    """
    try:
        result = invalidate_markdown_cache_task.apply(args=[request.post_id])
        
        if result.result["status"] == "SUCCESS":
            return {
                "success": True,
                "deleted": result.result["deleted"],
                "cache_key": result.result.get("cache_key"),
                "message": "캐시가 성공적으로 무효화되었습니다" if result.result["deleted"] else "무효화할 캐시가 없습니다"
            }
        else:
            raise HTTPException(
                status_code=500,
                detail=f"캐시 무효화 실패: {result.result.get('error', '알 수 없는 오류')}"
            )
            
    except Exception as e:
        logger.error(f"캐시 무효화 API 오류: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))


@router.post("/warm-up-cache", summary="마크다운 캐시 워밍업", tags=["markdown"])
async def warm_up_markdown_cache(request: MarkdownWarmupRequest) -> Dict[str, str]:
    """
    여러 마크다운 콘텐츠를 미리 렌더링하여 캐시를 워밍업합니다.
    비동기로 실행되며 태스크 ID를 반환합니다.
    """
    try:
        task = warm_up_markdown_cache_task.delay(request.posts, request.cache_ttl)
        
        return {
            "success": True,
            "task_id": task.id,
            "message": f"{len(request.posts)}개 마크다운 캐시 워밍업이 시작되었습니다"
        }
        
    except Exception as e:
        logger.error(f"캐시 워밍업 API 오류: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))


@router.get("/task-status/{task_id}", summary="태스크 상태 조회", tags=["markdown"])
async def get_task_status(task_id: str) -> TaskStatusResponse:
    """
    마크다운 관련 태스크의 상태를 조회합니다.
    """
    try:
        from app.core.celery_app import celery_app
        
        task = celery_app.AsyncResult(task_id)
        
        return TaskStatusResponse(
            task_id=task_id,
            status=task.status,
            result=task.result if task.ready() else None,
            meta=task.info if not task.ready() else None
        )
        
    except Exception as e:
        logger.error(f"태스크 상태 조회 API 오류: {str(e)}")
        raise HTTPException(status_code=500, detail=str(e))


@router.get("/health", summary="마크다운 서비스 헬스 체크", tags=["markdown"])
async def health_check() -> Dict[str, str]:
    """
    마크다운 캐시 서비스의 헬스 체크
    """
    try:
        from app.core.config import settings
        import redis
        
        # Redis 연결 테스트
        redis_client = redis.from_url(settings.REDIS_BACKEND_URL)
        redis_client.ping()
        
        return {
            "status": "healthy",
            "message": "마크다운 캐시 서비스가 정상적으로 작동 중입니다"
        }
        
    except Exception as e:
        logger.error(f"헬스 체크 실패: {str(e)}")
        raise HTTPException(status_code=503, detail=f"서비스 비정상: {str(e)}")