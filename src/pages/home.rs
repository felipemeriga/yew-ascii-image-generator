use crate::components::{display::Display, upload::Upload};
use crate::components::{
    error::Error as ErrorComp, error::ERROR_TIMEOUT_DISPLAY, loading::Loading,
};
use crate::utils::image::image_process;
use gloo::timers::callback::Timeout;
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_effect, use_state, Callback, Event, Html, TargetCast,
    UseStateHandle,
};

#[function_component]
#[allow(clippy::redundant_closure)]
pub fn Home() -> Html {
    let image_data: UseStateHandle<Vec<u8>> = use_state(|| vec![]);
    let ascii_string: UseStateHandle<String> = use_state(|| String::new());
    let error: UseStateHandle<String> = use_state(|| String::default());
    let loading: UseStateHandle<bool> = use_state(|| false);

    let error_clone = error.clone();
    use_effect(move || {
        if error_clone.to_string() != String::default() {
            let error_temp = error_clone.clone();
            Timeout::new(ERROR_TIMEOUT_DISPLAY, move || {
                error_temp.set(String::default());
            })
            .forget();
        }
    });

    let on_files_added = {
        let image_data = image_data.clone();
        let ascii_string = ascii_string.clone();
        let error = error.clone();
        let loading = loading.clone();

        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(file_list) = input.files() {
                loading.set(true);
                let image_data = image_data.clone();
                let ascii_string = ascii_string.clone();
                let error = error.clone();
                let loading = loading.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    match image_process(file_list).await {
                        Ok(image_result) => {
                            loading.set(false);
                            image_data.set(image_result.data);
                            ascii_string.set(image_result.ascii);
                        }
                        Err(err) => {
                            loading.set(false);
                            error.set(err.to_string())
                        }
                    }
                });
            }
        })
    };

    html!(
        <div id="wrapper">
            <ErrorComp message={error.to_string()}/>
            <Loading loading={*loading}/>
            <p id="title">{ "ASCII Image Transformer" }</p>
            <Upload {on_files_added} multiple={false}/>
            <Display image_data={image_data.to_vec()} ascii_string={ascii_string.to_string()}/>
        </div>
    )
}
