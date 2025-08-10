import logging
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, Optional, Tuple
from enum import Enum

import emails  # type: ignore
from jinja2 import Template

from app.core.config import settings

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


@dataclass
class EmailData:
    html_content: str
    subject: str


def render_email_template(*, template_name: str, context: dict[str, Any]) -> str:
    template_str = (
        Path(__file__).parent / "email-templates" / "build" / template_name
    ).read_text()
    html_content = Template(template_str).render(context)
    return html_content


def send_email(
    *,
    email_to: str,
    subject: str = "",
    html_content: str = "",
) -> None:
    assert settings.emails_enabled, "no provided configuration for email variables"
    message = emails.Message(
        subject=subject,
        html=html_content,
        mail_from=(settings.EMAILS_FROM_NAME, settings.EMAILS_FROM_EMAIL),
    )
    smtp_options = {"host": settings.SMTP_HOST, "port": settings.SMTP_PORT}
    if settings.SMTP_TLS:
        smtp_options["tls"] = True
    elif settings.SMTP_SSL:
        smtp_options["ssl"] = True
    if settings.SMTP_USER:
        smtp_options["user"] = settings.SMTP_USER
    if settings.SMTP_PASSWORD:
        smtp_options["password"] = settings.SMTP_PASSWORD
    response = message.send(to=email_to, smtp=smtp_options)
    logger.info(f"send email result: {response}")


def generate_test_email(email_to: str) -> EmailData:
    project_name = settings.PROJECT_NAME
    subject = f"{project_name} - Test email"
    html_content = render_email_template(
        template_name="test_email.html",
        context={"project_name": settings.PROJECT_NAME, "email": email_to},
    )
    return EmailData(html_content=html_content, subject=subject)


def generate_reset_password_email(email_to: str, email: str, token: str) -> EmailData:
    project_name = settings.PROJECT_NAME
    subject = f"{project_name} - Password recovery for user {email}"
    link = f"{settings.FRONTEND_HOST}/reset-password?token={token}"
    html_content = render_email_template(
        template_name="reset_password.html",
        context={
            "project_name": settings.PROJECT_NAME,
            "username": email,
            "email": email_to,
            "valid_hours": settings.EMAIL_RESET_TOKEN_EXPIRE_HOURS,
            "link": link,
        },
    )
    return EmailData(html_content=html_content, subject=subject)


def generate_new_account_email(
    email_to: str, username: str, password: str
) -> EmailData:
    project_name = settings.PROJECT_NAME
    subject = f"{project_name} - New account for user {username}"
    html_content = render_email_template(
        template_name="new_account.html",
        context={
            "project_name": settings.PROJECT_NAME,
            "username": username,
            "password": password,
            "email": email_to,
            "link": settings.FRONTEND_HOST,
        },
    )
    return EmailData(html_content=html_content, subject=subject)


# Task utilities


class TaskStatus(Enum):
    """태스크 상태 상수"""

    SUCCESS = "SUCCESS"
    FAILURE = "FAILURE"
    PARTIAL_SUCCESS = "PARTIAL_SUCCESS"
    WARNING = "WARNING"


class TaskError(Exception):
    """태스크 관련 커스텀 예외"""

    def __init__(self, message: str, error_code: Optional[str] = None):
        self.message = message
        self.error_code = error_code
        super().__init__(self.message)


class ValidationError(TaskError):
    """검증 관련 예외"""

    pass


class FileProcessingError(TaskError):
    """파일 처리 관련 예외"""

    pass


class DatabaseError(TaskError):
    """데이터베이스 관련 예외"""

    pass


def create_task_response(
    status: TaskStatus, message: str, **kwargs: Any
) -> Dict[str, Any]:
    """표준화된 태스크 응답 생성"""
    response = {
        "status": status.value,
        "message": message,
    }
    response.update(kwargs)
    return response


def create_success_response(message: str, **kwargs: Any) -> Dict[str, Any]:
    """성공 응답 생성"""
    return create_task_response(TaskStatus.SUCCESS, message, **kwargs)


def create_failure_response(error: str, **kwargs: Any) -> Dict[str, Any]:
    """실패 응답 생성"""
    return create_task_response(
        TaskStatus.FAILURE, f"작업 실패: {error}", error=error, **kwargs
    )


def validate_image_type(content_type: str, supported_types: set) -> None:
    """이미지 타입 검증"""
    if not content_type or content_type.lower() not in supported_types:
        raise ValidationError(
            f"지원되지 않는 이미지 타입: {content_type}. "
            f"지원되는 타입: {', '.join(supported_types)}"
        )


def validate_file_size(file_data: bytes, max_size: int) -> None:
    """파일 크기 검증"""
    if len(file_data) == 0:
        raise ValidationError("빈 파일입니다")

    if len(file_data) > max_size:
        raise ValidationError(
            f"파일이 너무 큽니다: {len(file_data)} bytes (최대: {max_size} bytes)"
        )


def validate_uuid(uuid_string: str) -> None:
    """UUID 형식 검증"""
    import uuid

    try:
        uuid.UUID(uuid_string)
    except (ValueError, TypeError):
        raise ValidationError(f"잘못된 UUID 형식: {uuid_string}")


def extract_filename_from_url(url: str) -> str:
    """URL에서 파일명 추출"""
    try:
        return url.split("/")[-1]
    except (AttributeError, IndexError):
        raise ValidationError(f"URL에서 파일명을 추출할 수 없습니다: {url}")


def safe_execute(func, *args, **kwargs) -> Tuple[bool, Any]:
    """안전한 함수 실행 (에러를 잡아서 로깅)"""
    try:
        result = func(*args, **kwargs)
        return True, result
    except Exception as e:
        logger.error(f"함수 실행 실패 {func.__name__}: {str(e)}")
        return False, str(e)
