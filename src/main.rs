use yew::prelude::*;

mod nav;
mod repl;
mod eval;
use nav::Navbar;
use repl::Evaluator;

#[function_component]
fn Content() -> Html {
    html! {
        <>
            <h1>{"Calculator!"}</h1>
            <p>{"This is a simple calculator that can evaluate basic arithmetic expressions."}</p>
            <ul>
                <li>{"+, -, *, / and basic order of operations"}</li>
                <li>{"Parentheses for grouping expressions"}</li>
                <li>{"Floating point numbers"}</li>
                <li>{"Negative numbers and negative grouping (e.g. -3, -(3 + 4))"}</li>
            </ul>
            <Evaluator />
        </>
    }
}
#[function_component]
fn App() -> Html {
    html! {
        <div class="main">
            <Navbar />
            <div class="content">
                <Content />
            </div>
            <footer class="footer">
                <p>{"\u{00A9} 2024 All rights reserved. "}</p>
            </footer>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
