from sqlalchemy import create_engine, select, update, delete, text
from sqlalchemy.orm import sessionmaker, Session
from app.core.config import settings
from app.models.user import User
from typing import Optional
import logging
from datetime import datetime

logger = logging.getLogger(__name__)

class DatabaseService:
    def __init__(self):
        # PostgreSQL 동기 연결 URL (psycopg2 드라이버 사용)
        self.database_url = f"postgresql://{settings.POSTGRES_USER}:{settings.POSTGRES_PASSWORD}@{settings.POSTGRES_HOST}:{settings.POSTGRES_PORT}/{settings.POSTGRES_NAME}"
        
        # 동기 엔진 생성
        self.engine = create_engine(
            self.database_url,
            echo=False,  # 개발 시에는 True로 설정하면 SQL 쿼리 로그를 볼 수 있음
            pool_size=5,
            max_overflow=10,
            pool_pre_ping=True  # 연결 상태 확인
        )
        
        # 동기 세션 팩토리
        self.session_factory = sessionmaker(
            self.engine,
            expire_on_commit=False
        )
    
    def get_session(self) -> Session:
        """데이터베이스 세션을 반환합니다."""
        return self.session_factory()
    
    def update_user_profile_image(self, user_handle: str, profile_image_url: Optional[str]) -> bool:
        """
        사용자의 프로필 이미지 URL을 업데이트합니다.
        
        Args:
            user_handle: 사용자 핸들
            profile_image_url: 새로운 프로필 이미지 URL (None이면 프로필 이미지 제거)
            
        Returns:
            bool: 업데이트 성공 여부
        """
        with self.session_factory() as session:
            try:
                # handle로 사용자 조회 후 profile_image 업데이트
                stmt = (
                    update(User)
                    .where(User.handle == user_handle)
                    .values(profile_image=profile_image_url)
                )
                
                result = session.execute(stmt)
                session.commit()
                
                if result.rowcount > 0:
                    action = "제거" if profile_image_url is None else "업데이트"
                    logger.info(f"사용자 프로필 이미지 {action} 성공: handle={user_handle}, url={profile_image_url}")
                    return True
                else:
                    logger.warning(f"사용자를 찾을 수 없음: handle={user_handle}")
                    return False
                    
            except Exception as e:
                logger.error(f"프로필 이미지 업데이트 실패: {str(e)}")
                session.rollback()
                return False
    
    def get_user_by_handle(self, user_handle: str) -> Optional[User]:
        """
        handle로 사용자를 조회합니다.
        
        Args:
            user_handle: 사용자 핸들
            
        Returns:
            Optional[User]: 사용자 객체 또는 None
        """
        with self.session_factory() as session:
            try:
                stmt = select(User).where(User.handle == user_handle)
                result = session.execute(stmt)
                user = result.scalar_one_or_none()
                
                if user:
                    logger.info(f"사용자 조회 성공: handle={user_handle}")
                    return user
                else:
                    logger.warning(f"사용자를 찾을 수 없음: handle={user_handle}")
                    return None
                    
            except Exception as e:
                logger.error(f"사용자 조회 실패: {str(e)}")
                return None
    
    def cleanup_expired_refresh_tokens(self, current_time: datetime) -> dict:
        """
        만료되거나 폐기된 리프레시 토큰을 정리합니다.
        
        Args:
            current_time: 현재 시각 (UTC)
            
        Returns:
            dict: 정리 결과 (success, expired_count, revoked_count, error)
        """
        with self.session_factory() as session:
            try:
                # 만료된 토큰 삭제 (expires_at < current_time)
                expired_stmt = text("""
                    DELETE FROM user_refresh_tokens 
                    WHERE expires_at < :current_time
                """)
                expired_result = session.execute(expired_stmt, {"current_time": current_time})
                expired_count = expired_result.rowcount
                
                # 폐기된 토큰 삭제 (revoked_at IS NOT NULL)
                revoked_stmt = text("""
                    DELETE FROM user_refresh_tokens 
                    WHERE revoked_at IS NOT NULL
                """)
                revoked_result = session.execute(revoked_stmt)
                revoked_count = revoked_result.rowcount
                
                session.commit()
                
                logger.info(f"토큰 정리 완료: 만료된 토큰 {expired_count}개, 폐기된 토큰 {revoked_count}개 삭제")
                
                return {
                    'success': True,
                    'expired_count': expired_count,
                    'revoked_count': revoked_count
                }
                
            except Exception as e:
                logger.error(f"토큰 정리 실패: {str(e)}")
                session.rollback()
                return {
                    'success': False,
                    'expired_count': 0,
                    'revoked_count': 0,
                    'error': str(e)
                }
    
    def close(self):
        """데이터베이스 연결을 종료합니다."""
        self.engine.dispose()

# 전역 데이터베이스 서비스 인스턴스
db_service = DatabaseService()