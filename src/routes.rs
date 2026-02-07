use dioxus::prelude::*;
use crate::components::layout::{SidebarLayout, PageNotFound};
use crate::views::{Inbox, Matrix, Calendar, Credits, Tutorial};

/// NAVIGATION MAP: This defines all the pages in our application and their web addresses
#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    // All of these pages are wrapped in the SidebarLayout (Common navigation menu)
    #[layout(SidebarLayout)]
        #[route("/")] // The default landing page
        Inbox {},
        
        #[route("/matrix")] // The Eisenhower Matrix priority view
        Matrix {},
        
        #[route("/calendar")] // The weekly time-blocking view
        Calendar {},
        
        #[route("/credits")] // About the creators
        Credits {},
        
        #[route("/tutorial")] // How to use the app
        Tutorial {},
    #[end_layout]
    
    // ERROR HANDLER: Catch-all for any unknown addresses
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}
