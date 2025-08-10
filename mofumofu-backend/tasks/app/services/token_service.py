from sqlalchemy import text
from app.services.base_db_service import base_db_service
from datetime import datetime
import logging

logger = logging.getLogger(__name__)


class TokenService:
    """토큰 관련 데이터베이스 서비스"""

    def __init__(self):
        self.db = base_db_service

    def cleanup_expired_refresh_tokens(self, current_time: datetime) -> dict:
        """
        만료되거나 폐기된 리프레시 토큰을 정리합니다.

        Args:
            current_time: 현재 시각 (UTC)

        Returns:
            dict: 정리 결과 (success, expired_count, revoked_count, error)
        """
        with self.db.session_factory() as session:
            try:
                # 만료된 토큰 삭제 (expires_at < current_time)
                expired_stmt = text("""
                    DELETE FROM user_refresh_tokens 
                    WHERE expires_at < :current_time
                """)
                expired_result = session.execute(
                    expired_stmt, {"current_time": current_time}
                )
                expired_count = expired_result.rowcount

                # 폐기된 토큰 삭제 (revoked_at IS NOT NULL)
                revoked_stmt = text("""
                    DELETE FROM user_refresh_tokens 
                    WHERE revoked_at IS NOT NULL
                """)
                revoked_result = session.execute(revoked_stmt)
                revoked_count = revoked_result.rowcount

                session.commit()

                logger.info(
                    f"토큰 정리 완료: 만료된 토큰 {expired_count}개, 폐기된 토큰 {revoked_count}개 삭제"
                )

                return {
                    "success": True,
                    "expired_count": expired_count,
                    "revoked_count": revoked_count,
                }

            except Exception as e:
                logger.error(f"토큰 정리 실패: {str(e)}")
                session.rollback()
                return {
                    "success": False,
                    "expired_count": 0,
                    "revoked_count": 0,
                    "error": str(e),
                }


# 전역 토큰 서비스 인스턴스
token_service = TokenService()
