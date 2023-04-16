use stylist::Style;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(Blog2)]
pub fn component() -> Html {
    let stylesheet = Style::new(crate::STYLE_FILE).unwrap();

    html! {
        <div class={classes!(stylesheet)}>
            <div class="blog-body">
                <div class={classes!("blog-content-container")}>
                    <h1>{ "Building a Website and Components in Yew" }</h1>
                    <h3 class="subtitle">{ "mood: I'm in love with the shape of "} <a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ">{ "Yew" }</a></h3>

                    <p>{ "In the previous blog post, we created a Yew application and deployed it to GitHub Pages. But the website our application serves is still just a stub/blank page. Let's change that." }</p>
                    <p>{ "In case you missed it, be sure to catch up on the first entry in this blog series:" }</p>
                    <h2><Link<Route> to={Route::Blog1}>{ "Creating and Deploying a Yew Application" }</Link<Route>></h2>

                    <h2>{ "Part 0: The Symbol of Death " }<span style="font-style: normal;">{ "💀" }</span></h2>
                    <p>{ "The Yew Tree, aka " } <a href="https://en.wikipedia.org/wiki/Taxus_baccata">{ "Taxus baccata" }</a> { ", is a tree native to Europe that has long been an emblem of death and often planted in or near graveyards. \"Most parts of the plant are poisonous, with toxins that can be absorbed through inhalation and through the skin; consumption of even a small amount of the foliage can result in death.\" I wonder if this is the source of inspiration for the Yew framework's name? 🤔" }</p>
                    <img src="img/blog2/yew-tree.webp" alt="image"/>
                    <h3>{ "But it looks so unnasuming " }<span style="font-style: normal;">{ "🥺" }</span></h3>

                    <h2>{ "Part 1: The Basics" }</h2>
                    <p>{ "wgirub" }</p>

                    <h2>{ "Part 2: Images" }</h2>
                    <p>{ "wgirub" }</p>

                    <h2>{ "Part 3: Routing" }</h2>
                    <p>{ "wgirub" }</p>

                    <h2>{ "Part 4: Component Seperation" }</h2>
                    <p>{ "wgirub" }</p>

                    <p>{ "And we're done. Thanks for following along." }</p>
                    <p class="code-snippet" style="margin-bottom: 20px;">{ "" }</p>

                    <h3>{ "Further Reading / Sources: " }</h3>
                    <h3 class="subtitle"><a href="https://en.wikipedia.org/wiki/Taxus_baccata">{ "Taxus baccata - Wikipedia" }</a></h3>
                    <h3 class="subtitle"><a href="https://www.theguardian.com/lifeandstyle/2020/jun/22/tree-of-the-week-sitting-in-this-yew-was-like-being-in-the-belly-of-a-large-creature">{ "Tree of the Week - The Guardian" }</a></h3>
                    <h3 class="subtitle"><a href="">{ "Tree of the Week - The Guardian" }</a></h3>
                    <h3 class="subtitle" style="margin-bottom: 40px;"><a href="https://docs.github.com/en/actions/quickstart">{ "Quickstart Tutorial - GitHub Actions" }</a></h3>
                </div>
            </div>
        </div>
    }
}
