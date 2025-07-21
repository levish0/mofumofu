from fastapi import FastAPI
from fastapi.routing import APIRoute

from app.api.main import api_router
from app.core.config import settings


def custom_generate_unique_id(route: APIRoute) -> str:
    return f"{route.tags[0]}-{route.name}"


app = FastAPI(
    title="textura_tasks",
    openapi_url=f"{settings.API_STR}/openapi.json",
    generate_unique_id_function=custom_generate_unique_id,
)

app.include_router(api_router, prefix=settings.API_STR)
