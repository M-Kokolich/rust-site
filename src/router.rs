use crate::pages::{blog1::Blog1, blog2::Blog2, home::Home};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog1")]
    Blog1,
    #[at("/blog2")]
    Blog2,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::Blog1 => html! {
            <Blog1 />
        },
        Route::Blog2 => html! {
            <Blog2 />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
