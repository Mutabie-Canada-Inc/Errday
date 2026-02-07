use dioxus::prelude::*;
use crate::components::layout::{SidebarLayout, PageNotFound};
use crate::views::{Inbox, Matrix, Calendar, Credits, Tutorial};

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[layout(SidebarLayout)]
        #[route("/")]
        Inbox {},
        #[route("/matrix")]
        Matrix {},
        #[route("/calendar")]
        Calendar {},
        #[route("/credits")]
        Credits {},
        #[route("/tutorial")]
        Tutorial {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}
