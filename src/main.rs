use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let foo = "bar";

    html! {
        <>
            <h1>{ "Rust Website" }</h1>
            <div>
                <h3>{ "Made with Yew" }</h3>
                <p>{ "1" }</p>
                <p>{ "2" }</p>
                <p>{ "3" }</p>
                <p>{ "4" }</p>
            </div>
            <div>
                <p>{ foo }</p>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}