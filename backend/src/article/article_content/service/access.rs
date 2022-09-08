use share::article::article_complete::ArticleCompleteHttp;

pub struct ArticleContentAccessService {
    article: ArticleCompleteHttp,
}

impl ArticleContentAccessService {
    pub fn new(article: ArticleCompleteHttp) -> ArticleContentAccessService {
        ArticleContentAccessService {
            article,
        }
    }

    pub fn render_markdown(&mut self) -> &mut Self {
        if let Some(content) = self.article.content.as_ref() {
            self.article.content = Some(markdown::to_html(content.clone().as_str()));
        }

        self
    }

    pub fn done(self) -> ArticleCompleteHttp {
        self.article
    }
}

