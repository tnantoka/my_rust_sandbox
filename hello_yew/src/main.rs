use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::prelude::*;

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

enum Msg {
    AddOne,
    SubOne,
    ChangeText(String),
}

struct Model {
    value: i64,
    text: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            text: String::from("hello"),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
            Msg::SubOne => {
                self.value -= 1;
                true
            }
            Msg::ChangeText(text) => {
                self.text = text;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
                <div>
                    <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                    { " " }
                    <button onclick={link.callback(|_| Msg::SubOne)}>{ "-1" }</button>
                    <p>{ self.value }</p>
                </div>
                <div>
                    <p>
                      <input
                          type="text"
                          value={self.text.clone()}
                          oninput={link.callback(|e| Msg::ChangeText(get_value_from_input_event(e)))}
                      />
                    </p>
                    <p>{self.text.to_uppercase()}</p>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
