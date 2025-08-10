from sqlalchemy import select, update, text
from app.models.post import Post
from app.services.base_db_service import base_db_service
from typing import Optional, List, Dict, Any
import logging

logger = logging.getLogger(__name__)


class PostService:
    """포스트 관련 데이터베이스 서비스"""

    def __init__(self):
        self.db = base_db_service

    def get_post_by_id(self, post_id: str) -> Optional[Post]:
        """
        post_id로 포스트를 조회합니다.

        Args:
            post_id: 포스트 UUID

        Returns:
            Optional[Post]: 포스트 객체 또는 None
        """
        with self.db.session_factory() as session:
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

    def update_post_thumbnail(self, post_id: str, thumbnail_url: Optional[str]) -> bool:
        """
        포스트의 썸네일 이미지 URL을 업데이트합니다.

        Args:
            post_id: 포스트 UUID
            thumbnail_url: 새로운 썸네일 이미지 URL (None이면 썸네일 제거)

        Returns:
            bool: 업데이트 성공 여부
        """
        with self.db.session_factory() as session:
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

    def get_all_posts_for_indexing(self) -> List[Dict[str, Any]]:
        """
        Meilisearch 색인을 위해 모든 포스트 데이터를 조회합니다.

        Returns:
            List[Dict[str, Any]]: 색인용 포스트 데이터 리스트
        """
        with self.db.session_factory() as session:
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
        with self.db.session_factory() as session:
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


# 전역 포스트 서비스 인스턴스
post_service = PostService()
