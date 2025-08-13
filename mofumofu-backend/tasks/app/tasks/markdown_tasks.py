import asyncio
import redis
import json
import hashlib
from celery import current_task
from app.core.celery_app import celery_app
from app.core.config import settings
import logging
import httpx
from typing import Optional, Dict, Any
from app.utils import create_failure_response

logger = logging.getLogger(__name__)

# Redis 백엔드 캐시 클라이언트 (DB 1 사용)
redis_cache = redis.from_url(settings.REDIS_BACKEND_URL, decode_responses=True)


def _get_cache_key(post_id: str) -> str:
    """포스트 ID로부터 캐시 키 생성"""
    return f"markdown:rendered:post:{post_id}"


@celery_app.task(bind=True, name="render_and_cache_markdown")
def render_and_cache_markdown_task(self, post_id: str, content: str, cache_ttl: int = 86400) -> Dict[str, Any]:
    """
    마크다운을 렌더링하고 Redis에 캐시
    
    Args:
        post_id: 포스트 ID
        content: 마크다운 콘텐츠
        cache_ttl: 캐시 유효시간 (초, 기본 24시간)
    
    Returns:
        dict: 렌더링 결과 또는 에러
    """
    try:
        cache_key = _get_cache_key(post_id)
        
        # 캐시에서 먼저 확인
        cached_result = redis_cache.get(cache_key)
        if cached_result:
            logger.info(f"마크다운 렌더링 캐시 히트: post_id={post_id}")
            return {
                "status": "SUCCESS",
                "data": json.loads(cached_result),
                "cached": True,
                "cache_key": cache_key
            }
        
        # 태스크 상태 업데이트
        current_task.update_state(
            state="PROGRESS", 
            meta={"status": "마크다운 렌더링 중...", "content_length": len(content)}
        )
        
        # 비동기 함수를 동기적으로 실행
        result = asyncio.run(_render_markdown(content))
        
        # Redis에 캐시 저장
        redis_cache.setex(cache_key, cache_ttl, json.dumps(result))
        logger.info(f"마크다운 렌더링 완료 및 캐시 저장: post_id={post_id}")
        
        return {
            "status": "SUCCESS",
            "data": result,
            "cached": False,
            "cache_key": cache_key
        }
        
    except Exception as exc:
        logger.error(f"마크다운 렌더링 실패: {str(exc)}")
        current_task.update_state(state="FAILURE", meta={"error": str(exc)})
        return create_failure_response(str(exc))


@celery_app.task(bind=True, name="get_cached_markdown")
def get_cached_markdown_task(self, post_id: str) -> Optional[Dict[str, Any]]:
    """
    캐시된 마크다운 렌더링 결과 조회
    
    Args:
        post_id: 포스트 ID
    
    Returns:
        dict: 캐시된 결과 또는 None
    """
    try:
        cache_key = _get_cache_key(post_id)
        cached_result = redis_cache.get(cache_key)
        
        if cached_result:
            logger.info(f"캐시된 마크다운 결과 조회 성공: post_id={post_id}")
            return {
                "status": "SUCCESS",
                "data": json.loads(cached_result),
                "cache_key": cache_key
            }
        else:
            logger.info(f"캐시된 마크다운 결과 없음: post_id={post_id}")
            return {"status": "NOT_FOUND", "cache_key": cache_key}
            
    except Exception as exc:
        logger.error(f"캐시 조회 실패: {str(exc)}")
        return create_failure_response(str(exc))


@celery_app.task(bind=True, name="invalidate_markdown_cache")
def invalidate_markdown_cache_task(self, post_id: str) -> Dict[str, Any]:
    """
    특정 포스트의 마크다운 캐시 무효화
    
    Args:
        post_id: 포스트 ID
    
    Returns:
        dict: 무효화 결과
    """
    try:
        cache_key = _get_cache_key(post_id)
        result = redis_cache.delete(cache_key)
        
        if result:
            logger.info(f"마크다운 캐시 무효화 완료: post_id={post_id}")
            return {"status": "SUCCESS", "cache_key": cache_key, "deleted": True}
        else:
            logger.info(f"무효화할 캐시 없음: post_id={post_id}")
            return {"status": "SUCCESS", "cache_key": cache_key, "deleted": False}
            
    except Exception as exc:
        logger.error(f"캐시 무효화 실패: {str(exc)}")
        return create_failure_response(str(exc))


async def _render_markdown(content: str) -> Dict[str, Any]:
    """마크다운 서비스를 통한 렌더링"""
    
    # 마크다운 서비스 URL
    markdown_service_url = f"http://{settings.MARKDOWN_SERVICE_HOST}:{settings.MARKDOWN_SERVICE_PORT}"
    
    request_data = {
        "markdown": content
    }
    
    async with httpx.AsyncClient(timeout=30.0) as client:
        try:
            response = await client.post(
                f"{markdown_service_url}/render",
                json=request_data
            )
            
            if response.status_code != 200:
                raise Exception(f"마크다운 서비스 요청 실패: {response.status_code} - {response.text}")
            
            result = response.json()
            
            if not result.get("success"):
                error_msg = result.get("error", "알 수 없는 오류")
                raise Exception(f"마크다운 처리 실패: {error_msg}")
            
            data = result.get("data")
            if not data:
                raise Exception("마크다운 서비스 응답에서 데이터 누락")
            
            # 표준화된 형태로 반환
            return {
                "html_content": data.get("htmlContent", ""),
                "toc_items": data.get("tocItems", [])
            }
            
        except httpx.TimeoutException:
            raise Exception("마크다운 서비스 타임아웃")
        except httpx.RequestError as e:
            raise Exception(f"마크다운 서비스 연결 실패: {str(e)}")


@celery_app.task(bind=True, name="warm_up_markdown_cache")
def warm_up_markdown_cache_task(self, posts: list, cache_ttl: int = 86400) -> Dict[str, Any]:
    """
    여러 포스트의 마크다운을 미리 렌더링하여 캐시 워밍업
    
    Args:
        posts: 포스트 리스트 [{"post_id": "uuid", "content": "markdown"}]
        cache_ttl: 캐시 유효시간
    
    Returns:
        dict: 워밍업 결과
    """
    try:
        results = []
        for i, post in enumerate(posts):
            post_id = post.get("post_id")
            content = post.get("content")
            
            if not post_id or not content:
                logger.warning(f"잘못된 포스트 데이터: {post}")
                results.append({
                    "post_id": post_id,
                    "success": False,
                    "error": "Missing post_id or content"
                })
                continue
                
            current_task.update_state(
                state="PROGRESS",
                meta={
                    "status": f"캐시 워밍업 중... ({i+1}/{len(posts)}) post_id={post_id}",
                    "progress": (i+1) / len(posts) * 100
                }
            )
            
            # 개별 렌더링 및 캐시
            result = render_and_cache_markdown_task(post_id, content, cache_ttl)
            results.append({
                "post_id": post_id,
                "cache_key": _get_cache_key(post_id),
                "success": result["status"] == "SUCCESS"
            })
        
        successful_count = sum(1 for r in results if r["success"])
        
        return {
            "status": "SUCCESS",
            "total": len(posts),
            "successful": successful_count,
            "failed": len(posts) - successful_count,
            "results": results
        }
        
    except Exception as exc:
        logger.error(f"캐시 워밍업 실패: {str(exc)}")
        return create_failure_response(str(exc))