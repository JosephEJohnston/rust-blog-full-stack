use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use yew_agent::{Agent, AgentLink, Context, HandlerId};
use share::article::article_base::ArticleHttp;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    RequestArticle(ArticleHttp),
}

pub struct ArticleDispatcher {
    _link: AgentLink<ArticleDispatcher>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for ArticleDispatcher {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Request;
    type Output = ArticleHttp;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            _link: link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn handle_input(&mut self, _msg: Self::Input, _id: HandlerId) {
        todo!()
    }
}

