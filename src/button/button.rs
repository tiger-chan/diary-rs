use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
	pub on_click: Callback<MouseEvent>,
	pub children: Children,
}

#[function_component]
pub fn Button(props: &Props) -> Html {
	html![
		<button class="" onclick={props.on_click.clone()}>
			<div class=""></div>
			{props.children.clone()}
		</button>
	]
}
