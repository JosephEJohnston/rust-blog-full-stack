use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, QueryableByName, table};
use serde::{Deserialize, Serialize};
use share::article::article_complete::ArticleCompleteHttp;

table! {
    article_content (id) {
        id -> Nullable<Bigint>,
        article_id -> Bigint,
        content -> Text,
        status -> Tinyint,
        create_time -> Nullable<Datetime>,
        modify_time -> Nullable<Datetime>,
    }
}


#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, QueryableByName)]
#[diesel(table_name = article_content)]
pub struct ArticleContentDB {
    pub id: Option<i64>,

    pub article_id: i64,

    pub content: String,

    pub status: i8,

    pub create_time: Option<NaiveDateTime>,

    pub modify_time: Option<NaiveDateTime>,
}

impl From<&ArticleCompleteHttp> for ArticleContentDB {
    fn from(article: &ArticleCompleteHttp) -> Self {
        ArticleContentDB {
            id: None,
            article_id: article.id.unwrap().clone(),
            content: article.content.as_ref().unwrap().clone(),
            status: 0,
            create_time: None,
            modify_time: None
        }
    }
}