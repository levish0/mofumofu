from fastapi import APIRouter
from app.api.routes.user.oauth import router as oauth_router
from app.api.routes.user.avatar import router as avatar_router
from app.api.routes.user.banner import router as banner_router
from app.api.routes.common.status import router as status_router
from app.api.routes.token.cleanup import router as token_cleanup_router

api_router = APIRouter()

# 사용자 관련 라우터 추가
api_router.include_router(oauth_router)
api_router.include_router(avatar_router)
api_router.include_router(banner_router)

# 공통 기능 라우터 추가
api_router.include_router(status_router)

# 토큰 관련 라우터 추가
api_router.include_router(token_cleanup_router)
