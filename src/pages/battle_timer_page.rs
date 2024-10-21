use leptos::*;
use leptos_router::*;
use leptos::html::Div;
use leptos_use::{
    use_interval_fn,
    on_click_outside
};
use leptos_use::utils::Pausable;

use crate::components::{
    card::{
        RedCard2, BlueCard2
    },
    button::{
        Button1
    },
    container::InnerContainer,
    timer::{
       Border,
       NumberBoxBg,
       TwoCircle
    },
};

use crate::data::{
    round::{
        Round, RoundType, SubRound
    }
};

#[component]
pub fn BattleTimerPage() -> impl IntoView {
    let params = use_params_map();
    let round = move || params.with(|params| params.get("round").cloned().unwrap_or_default());
    let subround = move || params.with(|params| params.get("subround").cloned().unwrap_or_default());
    let red = move || params.with(|params| params.get("red").cloned().unwrap_or_default());
    let blue = move || params.with(|params| params.get("blue").cloned().unwrap_or_default());
    let (show_modal, set_show_modal) = create_signal(false);
    let modal_ref = create_node_ref::<Div>();
    let _ = on_click_outside(modal_ref, move |_| set_show_modal.set(false));
    let (interval, set_interval) = create_signal(1000_u64);
    let (minutes, set_minutes) = create_signal(4_u16);
    let (seconds, set_seconds) = create_signal(0_u16);
    let (flag, set_flag) = create_signal(true);
    let button_class = "border-yellow-300 font-bold rounded-sm shadow-lg hover:border-yellow-500 border py-2";
    let button_url_class = "text-yellow-300 hover:text-yellow-500 uppercase py-2 px-10 rounded-sm text-xl";

    let Pausable {
        pause,
        resume,
        is_active,
    } = use_interval_fn(
        move || {
            if seconds.get() == 0 && minutes.get() > 0 {
                set_minutes.set(minutes.get() - 1);
                set_seconds.set(59);
                set_flag.set(false)
            } else if seconds.get() > 0 && seconds.get() < 60 {
                set_seconds.set(seconds.get() - 1);
            }
        },
        interval,
    );
    pause();

    let red_url = format!("/result/{}/{}/red/{}", round(), subround(), red());
    let blue_url = format!("/result/{}/{}/blue/{}", round(), subround(), blue());
    view!{
        <InnerContainer>
            <div class="flex flex-col items-center space-y-20">
                <div class="flex space-x-12 items-center">
                    <a href={red_url}>
                        <RedCard2 text={red()} />
                    </a>


                    // Timer
                    <div class="flex flex-col items-center">
                    <p class="text-yellow-300 my-2 font-semibold uppercase text-2xl"> {round()} </p>
                    <p class="text-white mb-10 font-normal uppercase text-lg"> {subround()} </p>
                    <div class="relative">
                        <div class="opacity-80">
                            <Border />
                        </div>
                        <button on:click=move |_| set_show_modal.set(true)>
                        <p class="absolute left-[121px] top-[-15px] text-2xl font-semibold text-left text-[#ffdf00]">
                            "TIMER" 
                        </p>
                        </button>
                        <NumberBoxBg />
                        <div class="w-[261px] h-20">
                            <p class="w-20 h-20 absolute left-5 top-3 text-[54px] font-bold text-center text-[#f1f1f1]">
                                {minutes}
                            </p>
                            <p
      class="w-20 h-20 absolute left-[201px] top-3 text-[54px] font-bold text-center text-[#f1f1f1]"
    >
                                {seconds}
                            </p>
                            <div class="absolute left-36 top-8"> 
                                <TwoCircle />
                            </div>
                        </div>
                    </div>
                    </div>

                    <a href={blue_url}>
                        <BlueCard2 text={blue()} />
                    </a>
                </div>

                <div class="flex space-x-44">
                    <Button1 text="Back" url="/vs" />

                    <div class="text-white">
                    // middle button
                    <Show 
                        when=move || is_active.get()
                        fallback = move || {
                            let resume = resume.clone();
                            view!{
                                <button 
                                    class={button_class}
                                    on:click=move |_| {
                                        set_flag.set(false);
                                        resume();
                                    }> 
                                    <a class={button_url_class}>{
                                    if flag.get() || (minutes.get() == 0 && seconds.get() == 0 && flag.get()) {
                                        "Start"
                                    } else {
                                        "Resume"
                                    }
                                }</a></button>
                            }
                        }
                    >
                    {
                        let pause = pause.clone();
                        view! {
                            <button 
                                class={button_class}
                                on:click=move |_| pause()> 
                                <a class={button_url_class}>{
                                if (minutes.get() == 0 && seconds.get() == 0) && !flag.get() {
                                    "Start"
                                } else if !flag.get() || (seconds.get() != 0 || minutes.get() != 0) {
                                    "Pause"
                                } else {
                                    "Start"
                                }
                            }</a></button>
                        }
                    }
                    </Show>
                    </div>

                    <button 
                        class={button_class}
                        on:click=move |_|{
                            set_minutes.set(0);
                            set_seconds.set(0);
                            set_flag.set(true)
                        }>
                        <a class={button_url_class}>
                            "Stop"
                        </a>
                    </button>
                </div>
            </div>

            // modal
            <Show when=move || show_modal.get() fallback=|| ()>
                <div node_ref=modal_ref
                    class="bg-black fixed left-1/2 top1/2 transform -translate-x-1/2 -translate-y-1/2 w-[420px] max-w-full z-10"
                    style="box-shadow: inset 0px 4px 20px 3px #ffb200"
                >
                    <div class="bg-[var(--bg)] p-4 px-8 rounded border border-[var(--theme-popup-border)] shadow-[2px_2px_10px_rgba(10,10,10,0.1)]">
                        <button 
                            class="absolute top-2 right-4 m-0 font-bold text-yellow-400 hover:text-white"
                            title="Close"
                            on:click=move|_| set_show_modal.set(false)>
                            "X"
                        </button>
                        <div class="flex flex-col">
                            <p class="font-bold text-[1.4rem] mb-4 text-yellow-400">
                                "Set Timer"
                            </p>
                            <div class="flex">
                                <input type="number"
                                    on:input=move |e| {
                                        set_minutes.set(event_target_value(&e).parse().unwrap());
                                        set_flag.set(false);
                                    }
                                    min="0" 
                                    max="60"
                                    class="text-white rounded-lg bg-yellow-300 bg-opacity-10 cursor-text w-14 aspect-square flex items-center justify-center text-center focus:outline-none appearance-none"
                                />
                                <p class="px-3 text-white"> " : "</p>
                                <input type="number"
                                    on:input=move |e| {
                                        set_seconds.set(event_target_value(&e).parse().unwrap());
                                        set_flag.set(false);
                                    }
                                    min="0"
                                    max="60"
                                    class="text-white rounded-lg bg-yellow-300 bg-opacity-10 cursor-text w-14 aspect-square flex items-center justify-center text-center focus:outline-none appearance-none"
                                />
                            </div>
                        </div>
                    </div>
                </div>
            </Show>
        </InnerContainer>
    }
}



