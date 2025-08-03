from fastapi import APIRouter
from app.api.routes.oauth import router as oauth_router
from app.api.routes.avatar import router as avatar_router
from app.api.routes.banner import router as banner_router
from app.api.routes.common import router as common_router
from app.api.routes.token_cleanup import router as token_cleanup_router

api_router = APIRouter()

# OAuth 관련 라우터 추가
api_router.include_router(oauth_router)

# 아바타 관련 라우터 추가
api_router.include_router(avatar_router)

# 배너 관련 라우터 추가
api_router.include_router(banner_router)

# 공통 기능 라우터 추가
api_router.include_router(common_router)

# 토큰 정리 관련 라우터 추가
api_router.include_router(token_cleanup_router)
