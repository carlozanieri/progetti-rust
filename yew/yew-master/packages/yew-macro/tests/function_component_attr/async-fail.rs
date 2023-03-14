use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
struct Props {
    a: usize,
}

#[function_component(Comp)]
async fn comp(props: &Props) -> Html {
    html! {
        <p>
            { props.a }
        </p>
    }
}


fn main() {}
