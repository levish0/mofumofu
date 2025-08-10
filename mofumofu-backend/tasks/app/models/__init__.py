from .user import User
from .post import Post, post_hash_tags
from .hashtag import HashTag
from .comment import Comment
from .refresh_token import UserRefreshToken

# 모든 Base 클래스를 하나로 통합
from sqlalchemy.ext.declarative import declarative_base

Base = declarative_base()

# 모든 모델들을 통합된 Base로 설정
User.metadata = Base.metadata
Post.metadata = Base.metadata
HashTag.metadata = Base.metadata
Comment.metadata = Base.metadata
UserRefreshToken.metadata = Base.metadata
post_hash_tags.metadata = Base.metadata

__all__ = [
    'Base',
    'User',
    'Post', 
    'HashTag',
    'Comment',
    'UserRefreshToken',
    'post_hash_tags'
]