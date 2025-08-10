from sqlalchemy import Column, String
from sqlalchemy.dialects.postgresql import UUID
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import relationship
import uuid

Base = declarative_base()


class HashTag(Base):
    __tablename__ = "hash_tags"

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    name = Column(String(50), nullable=False, unique=True, index=True)
    
    # 관계 정의
    posts = relationship("Post", secondary="post_hash_tags", back_populates="hashtags")

    def __repr__(self):
        return f"<HashTag(name='{self.name}')>"