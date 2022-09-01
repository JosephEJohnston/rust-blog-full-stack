use crate::utils::sql::sql_conn::get_connection;
use diesel::{QueryDsl, RunQueryDsl};
use diesel::prelude::*;
use crate::tag::tag_relation::sql::model::tag_relation::dsl::*;
use crate::tag::tag_relation::sql::model::TagRelationDB;

pub fn list_tag_relation_sql(entity_id_list: Vec<i64>, entity_type_: i32) -> Option<Vec<TagRelationDB>> {
    let conn = &mut get_connection();

    let query_result = tag_relation
        .filter(entity_id.eq_any(entity_id_list))
        .filter(entity_type.eq(entity_type_))
        .load::<TagRelationDB>(conn);

    return if let Ok(res) = query_result {
        Some(res)
    } else {
        None
    };
}


#[test]
fn test() {
    println!("{:?}", list_tag_relation_sql(vec![1], 1));
}