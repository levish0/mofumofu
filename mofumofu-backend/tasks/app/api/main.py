from fastapi import APIRouter
from app.api.routes.profile import router as profile_router
from app.api.routes.token_cleanup import router as token_cleanup_router

api_router = APIRouter()

# 프로필 관련 라우터 추가
api_router.include_router(profile_router)

# 토큰 정리 관련 라우터 추가
api_router.include_router(token_cleanup_router)
