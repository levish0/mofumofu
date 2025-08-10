from sqlalchemy import Column, String, Boolean, Text
from sqlalchemy.dialects.postgresql import UUID
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import relationship
import uuid

Base = declarative_base()


class User(Base):
    __tablename__ = "users"

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    name = Column(String(20), nullable=False)
    handle = Column(String(20), nullable=False, unique=True, index=True)
    email = Column(String(254), nullable=False, unique=True)
    password = Column(Text, nullable=True)
    is_verified = Column(Boolean, nullable=False, default=False)
    profile_image = Column(Text, nullable=True)
    banner_image = Column(Text, nullable=True)
    
    # 관계 정의
    posts = relationship("Post", back_populates="user", lazy="select")
    refresh_tokens = relationship("UserRefreshToken", back_populates="user", lazy="select")

    def __repr__(self):
        return f"<User(handle='{self.handle}', email='{self.email}')>"
