use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
mod router;

use router::{switch, Route};

pub const STYLE_FILE: &str = include_str!("styles/blog.css");

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
