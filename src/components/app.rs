use leptos::{component, view, IntoView};
use leptos_meta::{Link, Stylesheet};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="/pkg/when3meet.css"/>
        <header>
            <h1 class="p-6 text-4xl">GML</h1>
        </header>
    }
}
