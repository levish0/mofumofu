from sqlalchemy import Column, String, Text, Integer, DateTime, ForeignKey, Table
from sqlalchemy.dialects.postgresql import UUID
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import relationship
from sqlalchemy.sql import func
import uuid

Base = declarative_base()

# 다대다 관계를 위한 연결 테이블
post_hash_tags = Table(
    'post_hash_tags',
    Base.metadata,
    Column('post_id', UUID(as_uuid=True), ForeignKey('posts.id'), primary_key=True),
    Column('hash_tag_id', UUID(as_uuid=True), ForeignKey('hash_tags.id'), primary_key=True)
)


class Post(Base):
    __tablename__ = "posts"

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    title = Column(String(80), nullable=False)
    thumbnail_image = Column(Text, nullable=True)
    summary = Column(String(500), nullable=True)
    content = Column(Text, nullable=False)
    user_id = Column(UUID(as_uuid=True), ForeignKey('users.id'), nullable=False)
    created_at = Column(
        DateTime(timezone=True), server_default=func.now(), nullable=False
    )
    updated_at = Column(DateTime(timezone=True), nullable=True)
    like_count = Column(Integer, default=0, nullable=False)
    comment_count = Column(Integer, default=0, nullable=False)
    view_count = Column(Integer, default=0, nullable=False)
    slug = Column(String(80), nullable=False)
    
    # 관계 정의
    user = relationship("User", back_populates="posts")
    hashtags = relationship("HashTag", secondary=post_hash_tags, back_populates="posts")
    comments = relationship("Comment", back_populates="post", lazy="select")
