use stylist::Style;
use yew::prelude::*;
use yew_router::prelude::*;

const STYLE_FILE: &str = include_str!("main.css");

#[derive(Clone, Routable, PartialEq)]
enum Route {
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

#[function_component(Blog1)]
fn blog1() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Blog2));

    let stylesheet = Style::new(STYLE_FILE).unwrap();

    html! {
        <div class={classes!(stylesheet)}>
            <div class="blog-body">
                <div class={classes!("main-content-container")}>
                    <h1>{ "Creating and Deploying a Yew Application" }</h1>
                    <h3>{ "aka. Stevie Wonder's "} <a href="https://www.youtube.com/watch?v=zOW2UfvWWAE">{ "\"Yew and Eye\"" }</a></h3>

                    <p>{ "The web is an inherently concurrent environment. Many users may connect to a single server, which must respond in kind and handle race conditions appropriately and with care. Rust, having being built from the ground up with concurrency in mind, is well-suited to applications in web development." }</p>
                    <p>{ "This blog post is part of a series that will explore the current landscape of web development tools utilising Rust's unique advantages in the domain of web development, as well as act as a tutorial on how to create a simple web app." }</p>

                    <h2>{ "Part 0: Rust in the browser?" }</h2>
                    <p>{ "The secret sauce that brings this all together is "} <a href="https://webassembly.org/">{ "WebAssembly" }</a> {". The specifics of how Wasm works is out of the scope of this blog, but TLDR: WebAssembly exists in an assembly-like form and a binary form, which can be set as a compilation target for your favourite programming languages including C++, Rust, etc." }</p>
                    <p>{ "WebAssembly doesn't include a garbage collector natively, which was a pain for some languages, however Rust's robust ownership and borrow checking system made for an expedient shortcut when integrating Rust with Wasm. As such, there are a " } <a href="https://www.makeuseof.com/rust-webassembly-frameworks/">{ "number" }</a> { " of WebAssembly frameworks for Rust." }</p>
                    <p>{ "Today, we will be looking at " } <a href="https://yew.rs/">{ "Yew" }</a> { "." }</p>
                    <h3>{ "and You too, cutie ;)" }</h3>

                    <h2>{ "Part 1: Creating our Project" }</h2>
                    <p>{ "We start by adding Wasm as a compilation target and installing Yew's recommended Wasm web application bundler for Rust, Trunk." }</p>
                    <p class="code-snippet">{ "rustup target add wasm32-unknown-unknown" }</p>
                    <p class="code-snippet">{ "cargo install --locked trunk" }</p>
                    <p>{ "Then we create our Rust project." }</p>
                    <h3>{ "In my case called rust-site." }</h3>
                    <p class="code-snippet">{ "cargo new rust-site" }</p>
                    <p class="code-snippet">{ "cd rust-site" }</p>
                    <p>{ "double check our installation is all good with:" }</p>
                    <p class="code-snippet">{ "cargo run" }</p>
                    <p>{ "Add Yew as a dependancy in cargo.toml:" }</p>
                    <p class="code-snippet">{ "yew = { version = \"0.20.0\", features = [\"csr\"] }" }</p>
                    <p>{ "Copy paste some sample project code from the " } <a href="https://yew.rs/docs/getting-started/build-a-sample-app#update-mainrs">{ "Yew docs" }</a>{ " into main.rs." }</p>

                    <p>{ "Create a new file, index.html, in the root of project folder. Similarly copy paste the snippet from the "} <a href="https://yew.rs/docs/getting-started/build-a-sample-app#create-indexhtml">{ "Yew docs" }</a> { "." }</p>
                    <p>{ "You can now view the web page locally with:" }</p>
                    <p class="code-snippet">{ "trunk serve --open" }</p>
                    <p>{ "We can also build our project with:" }</p>
                    <p class="code-snippet">{ "trunk build --release" }</p>

                    <h2>{ "Part 2: Deploying our Project" }</h2>
                    <p>{ "We will be deploying our project with Github Pages." }</p>
                    <p>{ "Create a git repo from the root of your project files with " } <span class="code-snippet">{ "git init" }</span> { "." }</p>
                    <p>{ "Do all the usual steps to add and commit your project files, then push them to remote." }</p>
                    <p>{ "When we built our project, notice that it added a " } <span class="code-snippet">{ ".html" }</span> {", "} <span class="code-snippet">{ ".wasm" }</span> {" and "} <span class="code-snippet">{ ".js" }</span> { " file to the " } <span class="code-snippet">{ "/dist" }</span> { " folder in our source branch." }</p>
                    <p>{ "These are the source files for our website that get read by the browser and displayed to the user. By default, GitHub Pages looks for source files in the root (" } <span class="code-snippet">{ "/" }</span> { "), a " } <span class="code-snippet">{ "/docs" }</span> { " folder or source files in either of these locations on the " } <span class="code-snippet">{ "gh-pages" }</span> { " branch of your repo as a " } <a href="https://docs.github.com/en/pages/getting-started-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site#about-publishing-sources">{ "publishing source" }</a> { "." } </p>
                    <p>{ "First, we create an empty " } <span class="code-snippet">{ "gh-pages" }</span> { " branch."}</p>
                    <p class="code-snippet">{ "git checkout --orphan gh-pages" }</p>
                    <p class="code-snippet">{ "git rm -rf ." }</p>
                    <p class="code-snippet">{ "git add ." }</p>
                    <p class="code-snippet">{ "git commit -m \"created gh-pages branch\"" }</p>
                    <p class="code-snippet">{ "git push -u origin gh-pages" }</p>
                    <p>{ "Don't forget to switch back to main." }</p>
                    <p class="code-snippet">{ "git checkout main" }</p>

                    <p>{ "Now let's enable GitHub Pages. Navigate to your repo on Github -> Settings -> Pages -> Build and Deployment. Tell GitHub you want to deploy from the root of the newly-crated " } <span class="code-snippet">{ "gh-pages" }</span> { " branch."}</p>
                    <p>{ "Your deplopyoment settings should look like this:" }</p>
                    <img src="img/gh-pages-deployment-settings.png" alt="image"/>
                    <p>{ "For a more comprehensive set-up guide, check out the " } <a href="https://docs.github.com/en/pages/getting-started-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site">{ "GitHub Pages docs" }</a>{ "." }</p>

                    <p>{ "But building and pushing source files to a seperate branch every time we make code changes sounds annoying. So let's set up some CI/CD with GitHub Actions." }</p>
                    <p>{ "We have to grant " } <a href="https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository#configuring-the-default-github_token-permissions">{ "write permissions" }</a> { " to workflows using our " } <span class="code-snippet">{ "GITHUB_TOKEN" }</span> { "." }</p>
                    <p>{ "Navigate to your repo on Github -> Settings -> Actions -> General -> Workflow permissions. Enable read and write permissions. The settings should look something like: " }</p>
                    <img src="img/gh-pages-workflow-permissions.png" alt="image"/>

                    <p>{ "Now let's create some workflows." }</p>
                    <p>{ "First, we create a " } <span class="code-snippet">{ ".github/workflows" }</span> { " folder in our repo." }</p>
                    <p>{ "Create a " } <span class="code-snippet">{ "continuous_integration.yml" }</span> { " workflow file in this folder." }</p>
                    <p>{ "You can use " } <a href="https://pastebin.com/Yh9hUXTw">{ "this yaml file" }</a> { " to test, format and run clippy on the project. This takes a couple minutes each time though, so if you're feeling adventurous you can use " } <a href="https://pastebin.com/9QB1JJ4e">{ "this one instead" }</a> { " during development." }</p>
                    <p>{ "Similarly, create a " } <span class="code-snippet">{ "continuous_deployment.yml" }</span> { " workflow file in the " } <span class="code-snippet">{ ".github/workflows" }</span> { " folder with " } <a href="https://pastebin.com/2fAHTmC9">{ "this yaml" }</a> { ". IMPORTANT: change the cname line, " } <span class="code-snippet">{ "cname: supa.fish" }</span> { " to your own domain, or remove it entirely to use GitHub's auto-generated url." }</p>
                    <p>{ "Now when we push changes to our repo, it will automatically be tested. If successful, Trunk will build the project and place the source files in " } <span class="code-snippet">{ "/dist" }</span> { ". Then the source files will be copied to the " } <span class="code-snippet">{ "gh-pages" }</span> { " branch. GitHub Pages will then deploy the site publically on the web." }</p>
                    <p>{ "By default, your site will be available at:" }</p>
                    <p class="code-snippet">{ "https://your-name.github.io/your-repo" }</p>
                    <p>{ "You can also use a custom domain name. In my case, I added a cname to the repo to point to a custom domain address. And similarly added a cname record to my domain provider to point to my GitHub pages link." }</p>

                    <h2>{ "What's next?" }</h2>
                    <p>{ "From this point, the world is our oyster 🦪, and we are its crab 🦀." }</p>
                    <p>{ "Now that we have set up a pipeline that deploys our website whenever we push changes to our repo, it is very easy to iterate quickly and make changes to our website whenever we like." }</p>
                    <p>{ "In case you didn't already guess, this blog post is hosted on a website that was made using the method as described above. But if you've followed the guide up the this point, your version will still have the stub code from the Yew docs. What about filling our page with content and styling? How does routing work? Find out in the next blog post: " }</p>
                    <h2 {onclick}><a href="https://supa.fish/blog2">{ "Building a Website and Components in Yew" }</a></h2>
                    <p class="code-snippet" style="margin-bottom: 40px;">{ "" }</p>
                    <h3>{ "Further Reading / Sources: " }</h3>
                    <h3><a href="https://rustwasm.github.io/docs/book/introduction.html">{ "Rust 🦀 and WebAssembly 🕸 - Book" }</a></h3>
                    <h3><a href="https://bevy-cheatbook.github.io/platforms/wasm.html">{ "Deploying Wasm with Rust - Bevy Cheatbook" }</a></h3>
                    <h3><a href="https://www.youtube.com/watch?v=P4LMfkFLRsI">{ "Rust & Wasm - No Boilererplate" }</a></h3>
                    <h3><a href="https://plippe.github.io/blog/2021/07/12/rust-wasm-github.html">{ "Rust Wasm Github - Plippe" }</a></h3>
                    <h3><a href="https://yew.rs/docs/tutorial">{ "Getting Started Tutorial - Yew" }</a></h3>
                    <h3 style="margin-bottom: 40px;"><a href="https://docs.github.com/en/actions/quickstart">{ "Quickstart Tutorial - GitHub Actions" }</a></h3>
                </div>
            </div>
        </div>
    }
}

#[function_component(Blog2)]
fn blog2() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Blog1));
    html! {
        <div>
            <h1>{ "Blog 2" }</h1>
            <button {onclick}>{ "Go to Blog1" }</button>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Supa Site" }</h1> },
        Route::Blog1 => html! {
            <Blog1 />
        },
        Route::Blog2 => html! {
            <Blog2 />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

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
