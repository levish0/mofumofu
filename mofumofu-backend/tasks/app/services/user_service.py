from sqlalchemy import select, update
from app.models.user import User
from app.services.base_db_service import base_db_service
from typing import Optional
import logging

logger = logging.getLogger(__name__)


class UserService:
    """사용자 관련 데이터베이스 서비스"""

    def __init__(self):
        self.db = base_db_service

    def update_user_profile_image(
        self, user_handle: str, profile_image_url: Optional[str]
    ) -> bool:
        """
        사용자의 프로필 이미지 URL을 업데이트합니다.

        Args:
            user_handle: 사용자 핸들
            profile_image_url: 새로운 프로필 이미지 URL (None이면 프로필 이미지 제거)

        Returns:
            bool: 업데이트 성공 여부
        """
        with self.db.session_factory() as session:
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
                    logger.info(
                        f"사용자 프로필 이미지 {action} 성공: handle={user_handle}, url={profile_image_url}"
                    )
                    return True
                else:
                    logger.warning(f"사용자를 찾을 수 없음: handle={user_handle}")
                    return False

            except Exception as e:
                logger.error(f"프로필 이미지 업데이트 실패: {str(e)}")
                session.rollback()
                return False

    def update_user_profile_image_by_uuid(
        self, user_uuid: str, profile_image_url: Optional[str]
    ) -> bool:
        """
        사용자의 프로필 이미지 URL을 UUID로 업데이트합니다.

        Args:
            user_uuid: 사용자 UUID
            profile_image_url: 새로운 프로필 이미지 URL (None이면 프로필 이미지 제거)

        Returns:
            bool: 업데이트 성공 여부
        """
        with self.db.session_factory() as session:
            try:
                # UUID로 사용자 조회 후 profile_image 업데이트
                stmt = (
                    update(User)
                    .where(User.id == user_uuid)
                    .values(profile_image=profile_image_url)
                )

                result = session.execute(stmt)
                session.commit()

                if result.rowcount > 0:
                    action = "제거" if profile_image_url is None else "업데이트"
                    logger.info(
                        f"사용자 프로필 이미지 {action} 성공: uuid={user_uuid}, url={profile_image_url}"
                    )
                    return True
                else:
                    logger.warning(f"사용자를 찾을 수 없음: uuid={user_uuid}")
                    return False

            except Exception as e:
                logger.error(f"프로필 이미지 업데이트 실패: {str(e)}")
                session.rollback()
                return False

    def get_user_by_uuid(self, user_uuid: str) -> Optional[User]:
        """
        UUID로 사용자를 조회합니다.

        Args:
            user_uuid: 사용자 UUID

        Returns:
            Optional[User]: 사용자 객체 또는 None
        """
        with self.db.session_factory() as session:
            try:
                stmt = select(User).where(User.id == user_uuid)
                result = session.execute(stmt)
                user = result.scalar_one_or_none()

                if user:
                    logger.info(f"사용자 조회 성공: uuid={user_uuid}")
                    return user
                else:
                    logger.warning(f"사용자를 찾을 수 없음: uuid={user_uuid}")
                    return None

            except Exception as e:
                logger.error(f"사용자 조회 실패: {str(e)}")
                return None

    def get_user_by_handle(self, user_handle: str) -> Optional[User]:
        """
        handle로 사용자를 조회합니다.

        Args:
            user_handle: 사용자 핸들

        Returns:
            Optional[User]: 사용자 객체 또는 None
        """
        with self.db.session_factory() as session:
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

    def update_user_banner_image_by_uuid(
        self, user_uuid: str, banner_image_url: Optional[str]
    ) -> bool:
        """
        사용자의 배너 이미지 URL을 UUID로 업데이트합니다.

        Args:
            user_uuid: 사용자 UUID
            banner_image_url: 새로운 배너 이미지 URL (None이면 배너 이미지 제거)

        Returns:
            bool: 업데이트 성공 여부
        """
        with self.db.session_factory() as session:
            try:
                # UUID로 사용자 조회 후 banner_image 업데이트
                stmt = (
                    update(User)
                    .where(User.id == user_uuid)
                    .values(banner_image=banner_image_url)
                )

                result = session.execute(stmt)
                session.commit()

                if result.rowcount > 0:
                    action = "제거" if banner_image_url is None else "업데이트"
                    logger.info(
                        f"사용자 배너 이미지 {action} 성공: uuid={user_uuid}, url={banner_image_url}"
                    )
                    return True
                else:
                    logger.warning(f"사용자를 찾을 수 없음: uuid={user_uuid}")
                    return False

            except Exception as e:
                logger.error(f"배너 이미지 업데이트 실패: {str(e)}")
                session.rollback()
                return False


# 전역 사용자 서비스 인스턴스
user_service = UserService()
