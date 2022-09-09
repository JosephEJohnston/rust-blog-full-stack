use share::article::article_complete::ArticleCompleteHttp;
use crate::article::http::enums::MarkOpt;

pub struct ArticleContentAccessService {
    article: ArticleCompleteHttp,
}

impl ArticleContentAccessService {
    pub fn new(article: ArticleCompleteHttp) -> ArticleContentAccessService {
        ArticleContentAccessService {
            article,
        }
    }

    pub fn render_markdown(&mut self, markdown_opt: MarkOpt) -> &mut Self {
        if !markdown_opt.do_render() {
            return self;
        }

        if let Some(content) = self.article.content.as_ref() {
            self.article.content = Some(markdown::to_html(content.clone().as_str()));
        }

        self
    }

    pub fn done(self) -> ArticleCompleteHttp {
        self.article
    }
}

