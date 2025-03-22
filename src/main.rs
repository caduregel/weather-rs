use yew::prelude::*;

struct TemperatureProps {
    temperature: i8,
}

#[function_component(temperature)]
fn temperature() -> Html {
    html! {
        <p>{"The weather at that location is: "}{temp}</p>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
