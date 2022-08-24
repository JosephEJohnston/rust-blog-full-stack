use share::article::article_base::ArticleHttp;

struct ArticleService<'a> {
    article_list: &'a mut Vec<ArticleHttp>,
}

impl <'a> ArticleService <'a> {
    pub fn new(article_list: &mut Vec<ArticleHttp>) -> ArticleService {
        ArticleService {
            article_list
        }
    }

    pub fn each_set_with_tag_list(&mut self) {

    }
}