use crate::entity::comments::{Column as CommentColumn, Entity as CommentEntity};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use sea_orm::sea_query::SimpleExpr::FunctionCall;
use uuid::Uuid;
use sea_orm::sea_query::Func;

pub async fn repository_increment_reply_count<C>(
    conn: &C,
    parent_comment_id: &Uuid,
) -> Result<(), sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    CommentEntity::update_many()
        .filter(CommentColumn::Id.eq(*parent_comment_id))
        .col_expr(CommentColumn::ReplyCount, CommentColumn::ReplyCount.into_expr().add(1))
        .exec(conn)
        .await?;

    Ok(())
}

pub async fn repository_decrement_reply_count<C>(
    conn: &C,
    parent_comment_id: &Uuid,
) -> Result<(), sea_orm::DbErr>
where
    C: ConnectionTrait,
{
  
    
    CommentEntity::update_many()
        .filter(CommentColumn::Id.eq(*parent_comment_id))
        .col_expr(
            CommentColumn::ReplyCount, 
            FunctionCall(Func::greatest([
                CommentColumn::ReplyCount.into_expr().sub(1),
                0.into()
            ]))
        )
        .exec(conn)
        .await?;

    Ok(())
}