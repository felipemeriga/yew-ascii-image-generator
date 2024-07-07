use yew::{function_component, html, Html, Properties};

pub const ERROR_TIMEOUT_DISPLAY: u32 = 5000;
#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(String::new())]
    pub message: String,
}

#[function_component]
pub fn Error(props: &Props) -> Html {
    html! {
        if props.message != String::default() {
            <div id="error-container">
                <p>{props.message.clone()}</p>
            </div>
        }
    }
}
