use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos::html::Div;
use leptos_use::{
    use_interval_fn,
    on_click_outside
};
use leptos_use::utils::Pausable;
use leptos_use::WatchPausableReturn;

use crate::{
    pages::{
        battle_timer_page::BattleTimerPage,
        result_page::ResultPage,
        vs_page::VsPage,
        first_page::FirstPage
    },
    components::{
        container::{
            OuterContainer
        }
    }
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let (round, set_round) = create_signal(String::new());
    let (subround, set_subround) = create_signal(String::new());
    view!{

        <Stylesheet id="leptos" href="/pkg/scoreboard-atc-crs" />
        <Title text="ATC Drone Scoreboard" />


        <Router>
            <main class="font-display">
                <OuterContainer>
                    <Routes>
                        <Route path="/test" view=Demo />
                        <Route path="" view=FirstPage />
                        <Route path="/vs/:round/:subround/:red/:blue" view=VsPage />
                        <Route path="/battle_timer/:round/:subround/:red/:blue" view=BattleTimerPage />
                        <Route path="/result/:round/:subround/:color/:name" view=ResultPage />
                        <Route path="/*any" view=Error404 />
                    </Routes>
                </OuterContainer>
            </main>
        </Router>
    }
}

#[component]
fn Error404() -> impl IntoView {
    view!{
        <h1> "Error 404" </h1>
    }
}

#[component]
fn Demo() -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(false);
    let modal_ref = create_node_ref::<Div>();
    let _ = on_click_outside(modal_ref, move |_| set_show_modal.set(false));

    let (interval, set_interval) = create_signal(1000_u64);
    let (index, set_index) = create_signal(10);
    let (minutes, set_minutes) = create_signal(4_u16);
    let (seconds, set_seconds) = create_signal(0_u16);
    let (flag, set_flag) = create_signal(true);

    let Pausable {
        pause,
        resume,
        is_active,
    } = use_interval_fn(
        move || {
            if seconds.get() == 0  && minutes.get() > 0{
                set_minutes.set(minutes.get() - 1);
                set_seconds.set(59);
            } else if seconds.get() > 0 && seconds.get() < 60 {
                set_seconds.set(seconds.get() - 1);
            }
            
            if index.get() > 0 {
                set_index.set(index.get() - 1);
                set_flag.set(false);
            }
        },
        interval,
    );
    pause();

    view! {
        <div class="text-white">
        <button on:click=move |_| set_show_modal.set(true)>"set timer"</button>
        <Show when=move || show_modal.get() fallback=|| ()>
            <div node_ref=modal_ref 
                class="bg-black fixed left-1/2 top-1/2 transform -translate-x-1/2 -translate-y-1/2 w-[420px] max-w-full z-10"
                style="box-shadow: inset 0px 4px 67px 3px #ffb200;">
                <div class="bg-[var(--bg)] p-4 px-8 rounded border border-[var(--theme-popup-border)] shadow-[2px_2px_10px_rgba(10,10,10,0.1)]">
                    <button
                        class="absolute top-2 right-4 m-0 font-bold text-yellow-400 hover:text-white"
                        title="Close"
                        on:click=move |_| set_show_modal.set(false)
                    >
                        "𝖷"
                    </button>
                    <div class="flex flex-col">
                    <p class="font-bold text-[1.4rem] mb-4 text-yellow-400">"Set Timer"</p>
                    <p class="mb-2">"Click outside this modal to close it."</p>
                    <input type="number" 
                        on:input=move |e| {
                            set_index.set(event_target_value(&e).parse().unwrap());
                        }
                        min="0"
                        max="60"
                        class="text-white rounded-lg bg-yellow-300 bg-opacity-10 cursor-text w-14 aspect-square flex items-center justify-center text-center focus:outline-none appearance-none"
                    />
                    </div>
                </div>
            </div>
        </Show>
        <hr />
        <p> {minutes} : {seconds}</p>
        <hr />
        <p> {index} </p>
        <hr />
        <p>
            "Interval:"
            <input
                prop:value=move || interval.get()
                on:input=move |e| set_interval.set(event_target_value(&e).parse().unwrap())
                type="number"
                placeholder="interval"
                class="text-black"
            />
        </p>
        <Show
            when=move || is_active.get()
            fallback=move || {
                let resume = resume.clone();
                view! { 
                    <button on:click=move |_| resume()>{
                        if flag.get() || index.get() == 0 {
                            "Start"
                        } else {
                            "Resume"
                        }
                    }
                    </button> 
                }
            }
        >

            {
                let pause = pause.clone();
                // Pause
                view! {
                    <button on:click=move |_| pause()>{
                        if index.get() == 0 && !flag.get(){
                            "Start"
                        } else {
                            "Pause"
                        }
                    }</button> 
                }
            }

        </Show>

        <button class="mx-10" on:click=move |_|{
            set_index.set(0);
        }>
            "Stop"
        </button>

        </div>
    }
}
