from fastapi import APIRouter
from app.api.routes.profile import router as profile_router

api_router = APIRouter()

# 프로필 관련 라우터 추가
api_router.include_router(profile_router)
