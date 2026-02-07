use dioxus::prelude::*;
use crate::components::layout::{SidebarLayout, PageNotFound};
use crate::views::{Inbox, Matrix, Calendar};

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[layout(SidebarLayout)]
        #[route("/")]
        Inbox {},
        #[route("/matrix")]
        Matrix {},
        #[route("/calendar")]
        Calendar {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}
