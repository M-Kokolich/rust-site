use stylist::Style;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(Blog2)]
pub fn component() -> Html {
    let stylesheet = Style::new(crate::STYLE_FILE).unwrap();

    let first_load = use_state(|| true);
    use_effect(move || {
        if *first_load {
            gloo_utils::document_element().set_scroll_top(0);
            first_load.set(false);
        }
    });

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
                    <h3>{ "But it looks so unassuming " }<span style="font-style: normal;">{ "🥺" }</span></h3>

                    <h2>{ "Part 1: Setting Up Routing" }</h2>
                    <p>{ "Our main function creates a new renderer and renders the App component." }</p>
                    <img src="img/blog2/main.png" alt="image" style="max-width: 500px"/>

                    <p>{ "Yew allows us to define a function component by using the " } <span class="code-snippet">{ "function_component" }</span> { " attribute marker." }</p>
                    <p>{ "We want our website to be able to render a couple different pages, so we lean on the " } <span class="code-snippet"><a href="https://yew.rs/docs/next/concepts/router">{ "yew-router" }</a></span> { " to help us out with that. It's worth noting that Yew lets us write Single Page Applications, so if you are familiar with SPA routing techniques from other frontend frameworks, similar concepts apply here too." } </p>
                    <p>{ "Here, our " } <span class="code-snippet">{ "App" }</span> { " component returns a " } <span class="code-snippet">{ "BrowserRouter" }</span> { " (which provides routing functionality to the application) with a " } <span class="code-snippet">{ "Switch" }</span> { " component (that renders the first child " } <span class="code-snippet">{ "Route" }</span> { " component that matches the current URL)." }</p>
                    <img src="img/blog2/app.png" alt="image" style="max-width: 500px"/>
                    <p>{ "We have to define this " } <span class="code-snippet">{ "Route" }</span> { " enum. So let's go ahead and do that:" }</p>
                    <img src="img/blog2/route-enum.png" alt="image" style="max-width: 500px"/>
                    <p>{ "We then use this enum to match the given route in our switch function." }</p>
                    <img src="img/blog2/switch-function.png" alt="image" style="max-width: 500px"/>

                    <p>{ "Here, we are defining which component should be rendered depending on the given " } <span class="code-snippet">{ "Route" }</span> { ". Of course, none of these components exist yet, so let's create those." }</p>

                    <h2>{ "Part 2: Pages" }</h2>
                    <p>{ "Here's a simple implementation for the " }<span class="code-snippet">{ "Home" }</span>{ " component:" }</p>
                    <img src="img/blog2/home.png" alt="image" style="max-width: 500px"/>
                    <p>{ "By now you've probably noticed tags that looks suspiciously like html. That's because it is in fact html, sitting right there in our Rust code. Thanks to the " }<span class="code-snippet">{ "html!" }</span>{ " macro, we can write html code declaratively and pass it back to be rendered." }</p>
                    <p>{ "We can also use the " }<span class="code-snippet">{ "classes!" }</span>{ " macro to add classes to our html elements. Of course, we could also use the conventional " }<span class="code-snippet">{ "class=\"my-class\"" }</span>{ " syntax as well." }</p>
                    <p>{ "For more info check out the ever-helpful "} <a href="https://yew.rs/docs/next/concepts/html">{ "Yew docs" }</a> {"." }</p>

                    <p>{ "At present, css is not directly integrated in Yew (although support is proposed for a future version). There exist a couple methods for "} <a href="https://yew.rs/docs/more/css">{ "styling" }</a> {" our components. I'll show you the solution I ended up with using the " } <a href="https://docs.rs/stylist/latest/stylist/index.html">{ "Stylist" }</a> { " crate." }</p>
                    <p>{ "First we update our dependancies in " }<span class="code-snippet">{ "Cargo.toml" }</span>{ "." }</p>
                    <p class="code-snippet">{ r#"stylist = {version = "0.11.0", features = ["yew", "parser"] }"# }</p>
                    <p>{ "It's often useful to seperate your styles into a seperate file. It helps keep the project organised, certainly, but in my case the primary motivating factor was having access to my typical IDE tools from within a dedicated css file. So let's create a css file somewhere in our project directory and define a const that points to it." }</p>
                    <img src="img/blog2/style-file-const.png" alt="image" style="max-width: 500px"/>
                    <p>{ "Then, we can create a stylesheet and pass it to the " }<span class="code-snippet">{ "classes!" }</span>{ " macro to import our css file into our html." }</p>
                    <img src="img/blog2/stylesheet.png" alt="image" style="max-width: 500px"/>

                    <h2>{ "Part 3: Images" }</h2>
                    <p>{ "But what if we want to render images as well? Here we have to do a little more setup." }</p>
                    <p>{ "Let's create an " }<span class="code-snippet">{ "img" }</span>{ " folder at the root of our project. We need to instruct Trunk to copy the contents of this folder to the " }<span class="code-snippet">{ "dist" }</span>{ " folder whenever the project is built. So we add the following link to the head of the " }<span class="code-snippet">{ "index.html" }</span>{ " file." }</p>
                    <p class="code-snippet">{ r#"<link data-trunk rel="copy-dir" href="/img">"# }</p>
                    <p>{ "Now we can import our image into our component and use it as a src attribute, as we would in normal html." }</p>
                    <p class="code-snippet">{ r#"<img src="/img/my-picture.png" alt="crab"/>"# }</p>

                    <h2>{ "Part 4: Routing on GitHub Pages" }</h2>
                    <p>{ "As mentioned, Yew is a Single Page Application. GitHub Pages, on the other hand, doesn't natively support SPA's. If you've tried to push the code as described until this point to GitHub, the home page will work, but any routes will throw a 404. What gives?" }</p>
                    <img src="img/blog2/github-404.png" alt="image"/>
                    <p>{ "Once again we return to the trusty "} <a href="https://yew.rs/docs/more/deployment#serving-indexhtml-as-fallback">{ "Yew docs" }</a> { ". The entire paragraph is worth the read, but long story short: GitHub Pages is expecting to serve static files. When I press enter in the address bar of my browser asking for route " }<span class="code-snippet">{ "/blog2" }</span>{ ", GitHub Pages is looking for a " }<span class="code-snippet">{ "/blog2/index.html" }</span>{ " file from the source files, which doesn't exist. We need GitHub Pages to serve the " }<span class="code-snippet">{ "index.html" }</span>{ " that's sitting at the root of the source files no matter what route we attempt to load the site from." }</p>
                    <img src="img/blog2/yew-server-config.png" alt="image"/>
                    <p>{ "To get the desired behaviour, we need to add our own " } <a href="https://docs.github.com/en/pages/getting-started-with-github-pages/creating-a-custom-404-page-for-your-github-pages-site">{ "custom 404 page"}</a> { " that GitHub will serve the user when it encounters a route it doesn't have the expected source files for. This page, once it loads, should have a script in it that takes the given url and redirects the user to the root." }</p>
                    <p>{ "Luckily, this is a known problem and "} <a href="https://github.com/rafgraph">{ "rafgraph" }</a> { " has written these scripts in "} <a href="https://github.com/rafgraph/spa-github-pages">{ "Single Page Apps for GitHub Pages" }</a> { "." }</p>
                    <p>{ "Let's create our " }<span class="code-snippet">{ "404.html" }</span>{ " file with " }<a href="https://pastebin.com/TJrdDfxP">{ "this" }</a>{ " code." }</p>
                    <p>{ "Then, we need to add " }<a href="https://pastebin.com/FQ52ZheK">{ "this script" }</a>{ " to the head of our root-level " }<span class="code-snippet">{ "index.html" }</span>{ ", which will look for a redirect in the query string and push the correct url into the browser's history without loading a new page." }</p>
                    <p>{ "Finally, we need to let Trunk know that we want our newly created " }<span class="code-snippet">{ "404.html" }</span>{ " file to be included in the source files of our built project. To do that, we add the following line to the head of our " }<span class="code-snippet">{ "index.html" }</span>{ "." }</p>
                    <p class="code-snippet">{ r#"<link data-trunk rel="copy-file" href="/404.html">"# }</p>
                    <p>{ "And that's it. We can now push our project to GitHub and find that navigating to different pages will work as expected." }</p>

                    <h2>{ "Part 4: Component Seperation" }</h2>
                    <p>{ "Until now all of our Rust code has been in " }<span class="code-snippet">{ "main.rs" }</span>{ ". This works, but can get unweildy with a growing project. It would be nice if we distributed our codebase into files that each have their own purpose." }</p>
                    <p>{ "Note: I scrapped the write-up for this section. Turns out it's just easier to understand the file hierarchy when you're staring at the whole picture. The source code for " }<a href="https://github.com/M-Kokolich/rust-site">{ "this project" }</a>{ " is available on GitHub. I would also recommend " }<a href="https://github.com/brooks-builds/full-stack-todo-rust-course/tree/main/frontend/rust/yew/solution/src">{ "this example project" }</a>{ ". For more information on modules in Rust, be sure to check out "} <a href="https://doc.rust-lang.org/rust-by-example/mod.html">{ "Rust By Example" }</a> { "." }</p>

                    <h2>{ "Part 5: Conclusion " }<span style="font-style: normal;">{ "🥳" }</span></h2>
                    <p>{ "You are now armed with all of the knowledge you need to build the very website you've been staring at this whole time." }</p>
                    <img src="img/blog2/kronk.gif" alt="kronk" style="max-width: 500px"/>
                    <p>{ "There's still a lot to explore with Yew. State management, hooks, struct components, "}<a href="https://yew.rs/docs/advanced-topics/server-side-rendering">{ "server-side rendering" }</a>{ " and heap more. But those will have to wait for a future entry in this series :)" }</p>
                    <p>{ "Now we're done, thanks for following along." }</p>
                    <h3 class="subtitle">{ "Now go touch grass or smth idk." }</h3>
                    <p class="code-snippet" style="margin-bottom: 20px;">{ "" }</p>

                    <h3>{ "Further Reading / Sources: " }</h3>
                    <h3 class="subtitle"><a href="https://en.wikipedia.org/wiki/Taxus_baccata">{ "Taxus baccata - Wikipedia" }</a></h3>
                    <h3 class="subtitle"><a href="https://www.theguardian.com/lifeandstyle/2020/jun/22/tree-of-the-week-sitting-in-this-yew-was-like-being-in-the-belly-of-a-large-creature">{ "Tree of the Week - The Guardian" }</a></h3>
                    <h3 class="subtitle"><a href="https://yew.rs/docs/next/concepts/router">{ "Router - Yew Docs" }</a></h3>
                    <h3 class="subtitle"><a href="https://docs.rs/stylist/latest/stylist/index.html">{ "Stylist Crate Docs" }</a></h3>
                    <h3 class="subtitle"><a href="https://yew.rs/docs/more/deployment#serving-indexhtml-as-fallback">{ "Deployment - Yew Docs" }</a></h3>
                    <h3 class="subtitle"><a href="https://docs.github.com/en/pages/getting-started-with-github-pages/creating-a-custom-404-page-for-your-github-pages-site">{ "Custom 404 Page - GitHub Pages Docs" }</a></h3>
                    <h3 class="subtitle"><a href="https://github.com/rafgraph/spa-github-pages">{ "SPA For GitHub Pages - rafgraph" }</a></h3>
                    <h3 class="subtitle" style="margin-bottom: 40px;"><a href="https://www.youtube.com/playlist?list=PLrmY5pVcnuE_R5qJ0o30eGw77bWmnrUtL">{ "Introduction to Yew.rs - Brooks Builds" }</a></h3>
                </div>
            </div>
        </div>
    }
}
