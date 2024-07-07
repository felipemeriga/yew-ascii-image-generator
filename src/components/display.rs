use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(vec![])]
    pub image_data: Vec<u8>,
    #[prop_or(String::new())]
    pub ascii_string: String,
}

#[function_component]
pub fn Display(props: &Props) -> Html {
    html! {
        <div>
            if !props.image_data.is_empty() {
                <div class="preview-area">
                    <div class="preview-tile">
                        <p class="preview-name">{ format!("{}","") }</p>
                        <div class="preview-media">
                            <img src={format!("data:{};base64,{}","image/png", STANDARD.encode(props.image_data.as_slice()))} />

                            <pre id="show">
                                {props.ascii_string.to_string()}
                            </pre>
                        </div>
                    </div>
                </div>
            }
        </div>
    }
}
