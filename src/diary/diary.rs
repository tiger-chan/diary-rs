use yew::prelude::*;

const STYLES: &str = include_str!("diary.css");

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
}

struct DiaryEntry {
	pub title: String,
	pub date: String,
	pub preview: String,
}

#[function_component]
pub fn Diary(_: &Props) -> Html {
	let data = vec![
		DiaryEntry {
			title: "My First Entry".to_owned(),
			date: "June 1, 2023".to_owned(),
			preview: "This is the preview of my first diary entry. It's a great day today!".to_owned(),
		},
		DiaryEntry {
			title: "Another Entry".to_owned(),
			date: "May 30, 2023".to_owned(),
			preview: "Today, I had a wonderful time exploring the park with my friends.".to_owned(),
		},
	];

	html![<div>
		<style>{STYLES}</style>
		<h2>{"Diary Entries"}</h2>
		<ul class="entry-list">
			{
				data.into_iter().map(|entry| {
					html![
						<li class="entry-list-item secondary" key={entry.date.as_str()}>
							<h2 class="entity-title">{entry.title.as_str()}</h2>
							<p class="entry-date">{entry.date.as_str()}</p>
							<p class="entry-content">{entry.preview.as_str()}</p>
							<a class={classes!("view-entry-button", "btn", "accent")} href="entry.html">{"View Entry"}</a>
						</li>
					]
				}).collect::<Html>()
			}
		</ul>
	</div>]
}