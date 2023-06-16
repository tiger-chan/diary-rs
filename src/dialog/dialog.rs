use wasm_bindgen::{__rt::IntoJsResult, prelude::*};
use web_sys::{console, window, HtmlDialogElement};
use yew::prelude::*;

const STYLES: &str = include_str!("dialog.css");

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub id: String,
}

#[function_component]
pub fn Dialog(props: &Props) -> Html {
    let onclick = use_callback(
        |_: MouseEvent, props| {
            let document = window()
                .expect_throw("window is undefined")
                .document()
                .expect_throw("document is undefined");
            let dialog: HtmlDialogElement = document
                .get_element_by_id(props.id.as_str())
                .expect_throw("Dialog should exist")
                .into_js_result()
                .unwrap()
                .into();

			dialog.show_modal().unwrap();
        },
        props.clone(),
    );

    html![<div class="add-dialog">
		<style>{STYLES}</style>
		<dialog class="add-dialog-dialog" id={props.id.clone()}>
			<form>
				<label for="entry-title">{"Title"}</label>
				<input type="text" id="entry-title" name="entry-title" required=true />
				<label for="entry-date">{"Date"}</label>
				<input type="text" id="entry-date" name="entry-date" required=true />
				<label for="entry-content">{"Content"}</label>
				<textarea id="entry-content" name="entry-content" required=true></textarea>
				<button type="submit">{"Save Entry"}</button>
			</form>
		</dialog>

		<button id={ props.id.clone() + "open-button" } onclick={onclick}>{"Add Entry"}</button>
	</div>]
}
