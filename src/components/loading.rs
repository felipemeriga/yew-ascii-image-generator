use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(false)]
    pub loading: bool,
}

#[function_component]
pub fn Loading(props: &Props) -> Html {
    html! {
        if props.loading {
            <div id="loading-container">
                <p>{"loading..."}</p>
            </div>
        }
    }
}
