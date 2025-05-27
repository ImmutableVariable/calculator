use yew::prelude::*;

#[function_component]
pub fn Navbar() -> Html {
    html! {
        <nav class="navbar">
            <div class="logo">
                <img src="https://github.com/ImmutableVariable.png" id="round_image" alt="Profile Picture" width="100" height="100"/>
            </div>
            <ul class="nav-links">
                <li>
                    <h4>
                        <a href="/">{"home"}</a>
                    </h4>
                </li>
                <li>
                    <h4>
                        <a href="/projects">{"projects"}</a>
                    </h4>
                </li>
                <li>
                    <h4><a href="https://github.com/ImmutableVariable">{"Github"}</a></h4>
                </li>
            </ul>
        </nav>
    }
}
