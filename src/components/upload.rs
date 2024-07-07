use yew::{function_component, html, Callback, Event, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub on_files_added: Callback<Event>,
    #[prop_or(false)]
    pub multiple: bool,
}

#[function_component]
pub fn Upload(props: &Props) -> Html {
    let select_image_title = if props.multiple {
        "Select images..."
    } else {
        "Select an image..."
    };

    let on_files_added = {
        let props_on_file_added = props.on_files_added.clone();
        Callback::from(move |e: Event| {
            e.prevent_default();
            props_on_file_added.emit(e);
        })
    };

    html!(
        <div>
            <label for="file-upload">
                <div id="drop-container">
                    <i class="fa fa-cloud-upload"></i>
                    <p>{select_image_title}</p>
                </div>
            </label>
            <input
                id="file-upload"
                type="file"
                accept="image/*"
                multiple={props.multiple}
                onchange={on_files_added}/>
        </div>
    )
}
