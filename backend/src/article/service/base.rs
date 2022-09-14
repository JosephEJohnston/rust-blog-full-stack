use std::collections::HashMap;
use share::article::Article;
use share::article::article_statistics::ArticleStatisticsHttp;
use share::article::http::ListArticleOptions;
use share::tag::tag_base::TagHttp;
use share::user::simple_user::SimpleUserHttp;
use share::utils::page::{Pagination};
use crate::article::article_content::sql::access::{get_article_content};
use crate::article::article_statistics::sql::access::list_article_statistics;
use crate::article::sql::access::count_article_sql;
use crate::tag::sql::access::list_tag_sql;
use crate::tag::sql::model::TagDB;
use crate::tag::tag_relation::sql::access::list_tag_relation_sql;
use crate::user::sql::access::list_user;

pub struct ArticleService<T: Article> {
    article_list: Vec<T>,
}

impl <T: Article> ArticleService <T> {
    pub fn new(article_list: Vec<T>) -> ArticleService<T> {
        ArticleService {
            article_list
        }
    }

    pub fn to_page(self, opts: ListArticleOptions) -> Pagination<Vec<T>> {
        let total_count = count_article_sql(opts.clone()).unwrap_or(-1);
        let page_size = opts.page.page_size;

        let page = Pagination {
            page: opts.page.page,
            total_page: (total_count / page_size as i64) + 1,
            page_size,
            data: self.article_list,
        };

        page
    }

    pub fn each_set_with_tag_list(&mut self) {
        let entity_id_list = self.article_list.iter()
            .map(|article| article.get_id())
            .collect();

        let tag_relation_list =
            if let Some(tag_relation_list) = list_tag_relation_sql(
                entity_id_list, 1) {
                tag_relation_list
            } else {
                Vec::new()
            };

        let tag_id_list = tag_relation_list
            .iter()
            .map(|r| r.tag_id)
            .collect();

        let tag_list =
            if let Some(tag_list) = list_tag_sql(tag_id_list) {
                tag_list
            } else {
                Vec::new()
            };

        let relation_map = tag_relation_list.iter()
            .fold(HashMap::new(), |mut map: HashMap<i64, Vec<i64>>, relation| {
                if map.contains_key(&relation.entity_id) {
                    map.get_mut(&relation.entity_id).unwrap().push(relation.tag_id);
                } else {
                    let init = vec![relation.tag_id];
                    map.insert(relation.entity_id, init);
                }

                map
            });

        let tag_map = tag_list.iter()
            .fold(HashMap::new(), |mut map, tag| {
                map.insert(tag.id.unwrap(), tag);

                map
            });

        for article in self.article_list.iter_mut() {
            if let Some(tag_id_list) = relation_map.get(&article.get_id()) {
                let tag_list = tag_id_list.iter()
                    .map(|id| tag_map.get(id).unwrap())
                    .map(|tag| <TagDB as Into<TagHttp>>::into((*tag).clone()))
                    .collect();

                article.set_tag_list(tag_list);
            }
        }
    }

    pub fn each_set_with_statistics(&mut self) {
        let article_id_list = self.article_list.iter()
            .map(|article| article.get_id())
            .collect();

        if let Some(data_list) = list_article_statistics(article_id_list) {
            let map: HashMap<i64, ArticleStatisticsHttp> = data_list.into_iter()
                .map(|data| data.into())
                .collect::<Vec<ArticleStatisticsHttp>>().into_iter()
                .fold(HashMap::new(), |mut map: HashMap<i64, ArticleStatisticsHttp>, data: ArticleStatisticsHttp| {
                    map.insert(data.article_id, data);

                    map
                });

            self.article_list.iter_mut()
                .for_each(|article| {
                    if let Some(statistics) = map.get(&article.get_id()) {
                        article.set_statistics((*statistics).clone());
                    }
                })
        }
    }

    pub fn each_set_with_user(&mut self) {
        let user_id_list = self.article_list.iter()
            .map(|article| article.get_user_id())
            .collect();

        if let Some(user_list) = list_user(user_id_list) {
            let map: HashMap<i64, SimpleUserHttp> = user_list.into_iter()
                .map(|user| user.into())
                .collect::<Vec<SimpleUserHttp>>().into_iter()
                .fold(HashMap::new(), |mut map: HashMap<i64, SimpleUserHttp>, user: SimpleUserHttp| {
                    map.insert(user.id, user);

                    map
                });

            self.article_list.iter_mut()
                .for_each(|article| {
                    let user = map.get(&article.get_user_id()).unwrap();

                    article.set_user(user.clone());
                })
        }
    }
}

pub struct ArticleSingleService<T: Article> {
    article: T,
}

impl <T: Article> ArticleSingleService<T> {
    pub fn new(article: T) -> ArticleSingleService<T> {
        ArticleSingleService {
            article,
        }
    }

    pub fn consume(self) -> T {
        self.article
    }

    pub fn set_tag_list(&mut self) -> &mut Self {
        if let Some(tag_relation_list) = list_tag_relation_sql(vec![self.article.get_id()], 1) {
            let tag_id_list = tag_relation_list
                .iter()
                .map(|r| r.tag_id)
                .collect();

            if let Some(tag_list) = list_tag_sql(tag_id_list) {
                let tag_list: Vec<TagHttp> = tag_list.into_iter()
                    .map(|tag| tag.into())
                    .collect();

                self.article.set_tag_list(tag_list);
            }
        }

        self
    }

    pub fn set_content(&mut self) -> &mut Self {
        if let Some(content) = get_article_content(self.article.get_id()) {
            self.article.set_content(content.content.clone());
        }

        self
    }
}