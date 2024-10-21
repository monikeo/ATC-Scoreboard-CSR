use leptos::*;
use leptos_router::*;

#[component]
pub fn Button1(text: &'static str, url: &'static str) -> impl IntoView {
    view!{
        <button class="border-yellow-300 text-black font-bold rounded-sm shadow-lg hover:border-yellow-500 border py-2">
            <a href={url} class="text-yellow-300 hover:text-yellow-500 uppercase py-2 px-10 rounded-sm text-xl">
                {text}
            </a>
        </button>
    }
}

#[component]
pub fn Button2(text: &'static str, url: ReadSignal<String>) -> impl IntoView {
    view!{
        <button class="border-yellow-300 text-black font-bold rounded-sm shadow-lg hover:border-yellow-500 border py-2">
            <a prop:href=url class="text-yellow-300 hover:text-yellow-500 uppercase py-2 px-10 rounded-sm text-xl">
                {text}
            </a>
        </button>
    }
}

#[component]
pub fn Button3(text: &'static str, url: String) -> impl IntoView {
    view!{
        <button class="border-yellow-300 text-black font-bold rounded-sm shadow-lg hover:border-yellow-500 border py-2">
            <a href={url} class="text-yellow-300 hover:text-yellow-500 uppercase py-2 px-10 rounded-sm text-xl">
                {text}
            </a>
        </button>
    }
}

