from sqlalchemy import Column, String, Boolean, Text
from sqlalchemy.dialects.postgresql import UUID
from sqlalchemy.ext.declarative import declarative_base
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
    
    def __repr__(self):
        return f"<User(handle='{self.handle}', email='{self.email}')>"