use yew::prelude::*;
use web_sys::HtmlTextAreaElement;

use crate::eval::{tokenize, parse, eval};

pub struct Evaluator {
    pub input_value: String,
    pub output_value: String,
}

pub enum Msg {
    UpdateInput(String),
    Evaluate,
}

impl Component for Evaluator {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input_value: String::new(),
            output_value: String::new(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateInput(value) => {
                self.input_value = value;
                true
            }
            Msg::Evaluate => {
                self.output_value = self.evaluate(&self.input_value);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="calculator">
                <textarea
                    class = "calculator-input"
                    value={self.input_value.clone()}
                    oninput={ctx.link().callback(|e: InputEvent| {
                        let input: HtmlTextAreaElement = e.target_unchecked_into();
                        Msg::UpdateInput(input.value())
                    })}>
                </textarea>
                <button onclick={ctx.link().callback(|_| Msg::Evaluate)}>{ "Evaluate" }</button>
                <div class="calculator-output">
                    <pre>{ &self.output_value }</pre>
                </div>
            </div>
        }
    }
}

impl Evaluator {
    fn evaluate(&self, input: &str) -> String {
        match tokenize(input) {
            Ok(tokens) => match parse(tokens) {
                Ok(ast) => format!("AST: {}\nEval: {}", ast, eval(&ast)),
                Err(e) => format!("Error: {}", e),
            },
            Err(e) => format!("Error: {}", e),
        }
    }
}
