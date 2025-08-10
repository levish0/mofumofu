from sqlalchemy import create_engine, select, update, text
from sqlalchemy.orm import sessionmaker, Session
from sqlalchemy.engine import Engine
from app.core.config import settings
from app.models.user import User
from app.models.post import Post
from typing import Optional, List, Dict, Any
import logging
from datetime import datetime

logger = logging.getLogger(__name__)


class DatabaseService:
    def __init__(self) -> None:
        # PostgreSQL 동기 연결 URL (psycopg2 드라이버 사용)
        self.database_url = f"postgresql://{settings.POSTGRES_USER}:{settings.POSTGRES_PASSWORD}@{settings.POSTGRES_HOST}:{settings.POSTGRES_PORT}/{settings.POSTGRES_NAME}"

        # 동기 엔진 생성
        self.engine: Engine = create_engine(
            self.database_url,
            echo=False,  # 개발 시에는 True로 설정하면 SQL 쿼리 로그를 볼 수 있음
            pool_size=5,
            max_overflow=10,
            pool_pre_ping=True,  # 연결 상태 확인
        )

        # 동기 세션 팩토리
        self.session_factory: sessionmaker[Session] = sessionmaker(
            self.engine, expire_on_commit=False
        )

    def get_session(self) -> Session:
        """데이터베이스 세션을 반환합니다."""
        return self.session_factory()

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
        with self.session_factory() as session:
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
        with self.session_factory() as session:
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

    def get_post_by_id(self, post_id: str) -> Optional[Post]:
        """
        post_id로 포스트를 조회합니다.

        Args:
            post_id: 포스트 UUID

        Returns:
            Optional[Post]: 포스트 객체 또는 None
        """
        with self.session_factory() as session:
            try:
                stmt = select(Post).where(Post.id == post_id)
                result = session.execute(stmt)
                post = result.scalar_one_or_none()

                if post:
                    logger.info(f"포스트 조회 성공: post_id={post_id}")
                    return post
                else:
                    logger.warning(f"포스트를 찾을 수 없음: post_id={post_id}")
                    return None

            except Exception as e:
                logger.error(f"포스트 조회 실패: {str(e)}")
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
        with self.session_factory() as session:
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

    def update_post_thumbnail(self, post_id: str, thumbnail_url: Optional[str]) -> bool:
        """
        포스트의 썸네일 이미지 URL을 업데이트합니다.

        Args:
            post_id: 포스트 UUID
            thumbnail_url: 새로운 썸네일 이미지 URL (None이면 썸네일 제거)

        Returns:
            bool: 업데이트 성공 여부
        """
        with self.session_factory() as session:
            try:
                # post_id로 포스트 조회 후 thumbnail_image 업데이트
                stmt = (
                    update(Post)
                    .where(Post.id == post_id)
                    .values(thumbnail_image=thumbnail_url)
                )

                result = session.execute(stmt)
                session.commit()

                if result.rowcount > 0:
                    action = "제거" if thumbnail_url is None else "업데이트"
                    logger.info(
                        f"포스트 썸네일 {action} 성공: post_id={post_id}, url={thumbnail_url}"
                    )
                    return True
                else:
                    logger.warning(f"포스트를 찾을 수 없음: post_id={post_id}")
                    return False

            except Exception as e:
                logger.error(f"포스트 썸네일 업데이트 실패: {str(e)}")
                session.rollback()
                return False

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

    def get_all_posts_for_indexing(self) -> List[Dict[str, Any]]:
        """
        Meilisearch 색인을 위해 모든 포스트 데이터를 조회합니다.

        Returns:
            List[Dict[str, Any]]: 색인용 포스트 데이터 리스트
        """
        with self.session_factory() as session:
            try:
                logger.info("포스트 데이터 조회 시작...")

                # 먼저 간단한 포스트 수 조회로 테스트
                count_stmt = text("SELECT COUNT(*) as total FROM posts")
                count_result = session.execute(count_stmt)
                total_posts = count_result.scalar()
                logger.info(f"전체 포스트 수: {total_posts}")

                if total_posts == 0:
                    logger.warning("데이터베이스에 포스트가 없습니다")
                    return []

                # 포스트와 사용자 정보, 해시태그를 조인하여 조회 (Rust 구조와 동일하게)
                stmt = text("""
                    SELECT 
                        p.id,
                        p.user_id,
                        u.handle as user_handle,
                        u.name as user_name,
                        u.profile_image as user_avatar,
                        p.title,
                        p.content,
                        p.summary,
                        p.slug,
                        p.thumbnail_image,
                        p.created_at,
                        p.like_count,
                        p.view_count,
                        p.comment_count,
                        ARRAY_AGG(DISTINCT ht.name) FILTER (WHERE ht.name IS NOT NULL) as hashtags
                    FROM posts p
                    LEFT JOIN users u ON p.user_id = u.id
                    LEFT JOIN post_hash_tags pht ON p.id = pht.post_id
                    LEFT JOIN hash_tags ht ON pht.hash_tag_id = ht.id
                    GROUP BY p.id, p.user_id, u.handle, u.name, u.profile_image, p.title, p.content, 
                             p.summary, p.slug, p.thumbnail_image, p.created_at, p.like_count, p.view_count, p.comment_count
                    ORDER BY p.created_at DESC
                """)

                logger.info("포스트 조회 쿼리 실행 중...")
                result = session.execute(stmt)
                posts = []

                row_count = 0
                for row in result:
                    row_count += 1
                    logger.info(f"포스트 {row_count}: id={row.id}, title={row.title}")
                    posts.append(
                        {
                            "id": str(row.id),
                            "user_id": str(row.user_id),
                            "user_handle": row.user_handle or "",
                            "user_name": row.user_name or "",
                            "user_avatar": row.user_avatar,
                            "title": row.title or "",
                            "content": row.content or "",
                            "summary": row.summary,
                            "slug": row.slug or "",
                            "thumbnail_image": row.thumbnail_image,
                            "hashtags": list(row.hashtags) if row.hashtags else [],
                            "created_at": row.created_at,
                            "like_count": row.like_count or 0,
                            "view_count": row.view_count or 0,
                            "comment_count": row.comment_count or 0,
                        }
                    )

                logger.info(f"색인용 포스트 데이터 조회 완료: {len(posts)}개")
                return posts

            except Exception as e:
                logger.error(f"색인용 포스트 데이터 조회 실패: {str(e)}")
                import traceback

                logger.error(f"스택 트레이스: {traceback.format_exc()}")
                return []

    def get_posts_by_ids(self, post_ids: List[str]) -> List[Dict[str, Any]]:
        """
        특정 포스트 ID들로 포스트 데이터를 조회합니다.

        Args:
            post_ids: 조회할 포스트 UUID 리스트

        Returns:
            List[Dict[str, Any]]: 색인용 포스트 데이터 리스트
        """
        with self.session_factory() as session:
            try:
                # 포스트 ID들을 플레이스홀더로 변환
                placeholders = ", ".join(
                    [f":post_id_{i}" for i in range(len(post_ids))]
                )

                # 포스트와 사용자 정보, 해시태그를 조인하여 조회
                stmt = text(f"""
                    SELECT 
                        p.id,
                        p.user_id,
                        u.handle as user_handle,
                        u.name as user_name,
                        u.profile_image as user_avatar,
                        p.title,
                        p.content,
                        p.summary,
                        p.slug,
                        p.thumbnail_image,
                        p.created_at,
                        p.like_count,
                        p.view_count,
                        p.comment_count,
                        ARRAY_AGG(DISTINCT ht.name) FILTER (WHERE ht.name IS NOT NULL) as hashtags
                    FROM posts p
                    LEFT JOIN users u ON p.user_id = u.id
                    LEFT JOIN post_hash_tags pht ON p.id = pht.post_id
                    LEFT JOIN hash_tags ht ON pht.hash_tag_id = ht.id
                    WHERE p.id IN ({placeholders})
                    GROUP BY p.id, p.user_id, u.handle, u.name, u.profile_image, p.title, p.content, 
                             p.summary, p.slug, p.thumbnail_image, p.created_at, p.like_count, p.view_count, p.comment_count
                    ORDER BY p.created_at DESC
                """)

                # 파라미터 생성
                params = {f"post_id_{i}": post_id for i, post_id in enumerate(post_ids)}

                result = session.execute(stmt, params)
                posts = []

                for row in result:
                    posts.append(
                        {
                            "id": str(row.id),
                            "user_id": str(row.user_id),
                            "user_handle": row.user_handle or "",
                            "user_name": row.user_name or "",
                            "user_avatar": row.user_avatar,
                            "title": row.title or "",
                            "content": row.content or "",
                            "summary": row.summary,
                            "slug": row.slug or "",
                            "thumbnail_image": row.thumbnail_image,
                            "hashtags": list(row.hashtags) if row.hashtags else [],
                            "created_at": row.created_at,
                            "like_count": row.like_count or 0,
                            "view_count": row.view_count or 0,
                            "comment_count": row.comment_count or 0,
                        }
                    )

                return posts

            except Exception as e:
                logger.error(f"특정 포스트 데이터 조회 실패: {str(e)}")
                import traceback

                logger.error(f"스택 트레이스: {traceback.format_exc()}")
                return []

    def close(self) -> None:
        """데이터베이스 연결을 종료합니다."""
        self.engine.dispose()


# 전역 데이터베이스 서비스 인스턴스
db_service = DatabaseService()
