use leptos::*;
use leptos_router::*;

use crate::{
    components::{
        container::InnerContainer,
        button::Button2
    }
};

use crate::data::{
    round::{
        Round, RoundType, SubRound
    }
};

#[component]
pub fn RadioBox(text: String, name: String, set_signal: WriteSignal<String>) -> impl IntoView {
    let value = text.clone();
    view!{
        <label class="cursor-pointer justify-center flex rounded-sm border border-white">
            <input type="radio" class="peer sr-only" name={name} 
                on:click=move |_| {
                    set_signal.set(value.clone())
                }
            />
            <div class="w-36 rounded-sm flex justify-center items-center peer-checked:text-yellow-300 text-white">
                <p class="text-sm font-semibold uppercase">
                    {text}
                </p>
            </div>
        </label>
    }
}

#[component]
pub fn CompetitorBox(text: String, set_signal: WriteSignal<String>, color: String) -> impl IntoView {
    let value = text.clone();
    let flag = "border-blue-500 border-red-500 peer-checked:text-red-500 peer-checked:text-blue-500";
    let class1 =  format!("my-4 mx-2 cursor-pointer justify-center flex rounded-sm border border-{}-500", &color);
    let class2 = format!("px-10 py-2 rounded-md flex justify-center items-center peer-checked:text-{}-500 text-white text-xl peer-checked:text-2xl", color);
    view!{
        <label class={class1} >
            <input type="radio" class="peer sr-only" name={color}
                on:click=move |_| {
                    set_signal.set(value.clone())
                }
            />
            <div class={class2}>
                <p class="font-semibold uppercase">
                    {text}
                </p>
            </div>
        </label>
    }
}

#[component]
pub fn CompetitorBoxRed(text: String, set_signal: WriteSignal<String>) -> impl IntoView {
    view!{
        <CompetitorBox text={text} set_signal={set_signal} color={"red".to_string()} /> 
    }
}

#[component]
pub fn CompetitorBoxBlue(text: String, set_signal: WriteSignal<String>) -> impl IntoView {
    view!{
        <CompetitorBox text={text} set_signal={set_signal} color={"blue".to_string()} />
    }
}

#[component]
pub fn FirstPage() -> impl IntoView {
    let (round, set_round) = create_signal(String::new());
    let (subround, set_subround) = create_signal(String::new());
    let (blue, set_blue) = create_signal(String::new());
    let (red, set_red) = create_signal(String::new());
    let go_on_click = move |_| {
        let round = round.get();
        let subround = subround.get();
        let red = red.get();
        let blue = blue.get();
        if !round.is_empty() && !subround.is_empty() && !red.is_empty() && !blue.is_empty(){
            let route = format!("/vs/{}/{}/{}/{}", round,subround, red, blue);
            let navigate = use_navigate();
            navigate(&route, Default::default());
        }
    };

    let team_reds = ["red1", "red2-sfcxv", "red3", "red4-2", "red5", "red6-sdjfif", "red7-23"];
    let team_blues = ["blue1", "blue2-ssfdsdffsdfw", "blue3-2", "blue4-23r", "blue5", "blue6-asjfj2sfd", "blue7"];
    view!{
        <InnerContainer>

            <div class="flex flex-col bg-black uppercase font-bold text-yellow-300 justify-center items-center w-screen h-screen">
                <div class="w-10/12 h-5/6 flex flex-col justify-between">
                    <div class="flex items-center justify-center w-full h-full">
                        <div class="w-1/2">
                            <div class="flex flex-wrap justify-around">
                                {team_reds.into_iter()
                                    .map(|n| view!{
                                        <CompetitorBoxRed text={n.to_string()} set_signal={set_red} />
                                    }).collect::<Vec<_>>()
                                }
                            </div>
                        </div>
                        <div class="h-full w-5"></div>
                        <div class="w-1/2">
                            <div class="flex flex-wrap justify-around">
                                {team_blues.into_iter()
                                    .map(|n| view!{
                                        <CompetitorBoxBlue text={n.to_string()} set_signal={set_blue} />
                                    }).collect::<Vec<_>>()
                                }
                            </div>
                        </div>
                    </div>

                    <div class="flex flex-col items-center justify-center w-full">
                        <hr class="w-full h-px my-8 bg-gray-200 border-0 dark:bg-gray-700" />
                        <div class="flex w-full">
                            <div class="flex flex-col items-start justify-start w-3/4">
                                <div class="flex justify-between w-full"> <h2 class="text-3xl w-1/3"> "Round" </h2> 
                                    <div class="flex flex-wrap justify-start gap-3 w-2/3">
                                        <RadioBox text={"Quarter-Final".to_string()} name={"round".to_string()} set_signal={set_round}/>
                                        <RadioBox text={"Semi-Final".to_string()} name={"round".to_string()} set_signal={set_round}/>
                                        <RadioBox text={"Final".to_string()} name={"round".to_string()} set_signal={set_round}/>
                                    </div>
                                </div>
                                <div class="py-3"> </div>
                                <div class="flex justify-between w-full">
                                    <h2 class="text-3xl w-1/3"> "Sub Round" </h2>
                                    <div class="flex flex-wrap justify-start gap-3 w-2/3">
                                        <RadioBox text={"Round-1".to_string()} name={"subround".to_string()} set_signal={set_subround}/>
                                        <RadioBox text={"Round-2".to_string()} name={"subround".to_string()} set_signal={set_subround}/>
                                    </div>
                                </div>
                            </div>

                            <div class="w-1/4 flex justify-end items-center">

                                <button class="border-yellow-300 text-black font-bold rounded-sm shadow-lg hover:border-yellow-500 border py-2">
                                    <a class="text-yellow-300 hover:text-yellow-500 uppercase py-2 px-10 rounded-sm text-xl"
                                        href="#" on:click=go_on_click
                                    >
                                        "Go"
                                    </a>
                                </button>

                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </InnerContainer>
    }
}
