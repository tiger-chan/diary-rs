mod button;
mod dialog;
mod diary;

use button::Button;
use diary::Diary;
use dialog::Dialog;
use yew::prelude::*;
use yew_router::prelude::*;

const CORE_STYLES: &str = include_str!("core.css");
const MAIN_STYLES: &str = include_str!("main.css");

#[derive(Routable, Clone, Debug, Eq, PartialEq)]
pub enum Route {
	#[at("/")]
	Home,
	#[at("/diary")]
	Diary,
}

#[function_component]
fn App() -> Html {
	html!{<div class="root-body">
		<style>
		 {CORE_STYLES}
		</style>
		<style>
		{MAIN_STYLES}
		</style>
		<header class="core-header primary-dark">
			<h1>{"Welcome to the Diary Entry App"}</h1>
		</header>
		<BrowserRouter>
			<Switch<Route> render={switch} />
		</BrowserRouter>
		<Dialog id="add-entry-dialog" />
	</div>}
}

#[function_component]
fn Home() -> Html {
	let counter = use_state(|| 0);
	let onclick = {
		let counter = counter.clone();
		move |_| {
			let value = *counter + 1;
			counter.set(value);
		}
	};

	html! {<div>
		<Button on_click={onclick}>{"+1"}</Button>
		<p>{*counter}</p>
		<p class="description">{"A simple and intuitive app to record your daily thoughts and reflections."}</p>
		<Link<Route> classes={classes!("btn", "accent")} to={Route::Diary}>{"Get Started"}</Link<Route>>
	</div>}
}

fn switch(routes: Route) -> Html {
	match routes {
		Route::Home => html!(<Home />),
		Route::Diary => html!(<Diary />)
	}
}

fn main() {
	yew::Renderer::<App>::new().render();
}

