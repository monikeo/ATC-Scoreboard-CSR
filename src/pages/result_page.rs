use leptos::*;
use leptos_router::*;

use crate::components::{
    button::Button1,
    card::{
        RedCard2, BlueCard2,ResultCard2
    }
};
use crate::data::round::{
    Round, RoundType, SubRound
};

#[component]
pub fn ResultPage() -> impl IntoView {
    let params = use_params_map();
    let round = move || params.with(|params| params.get("round").cloned().unwrap_or_default());
    let sub_round = move || params.with(|params| params.get("subround").cloned().unwrap_or_default());
    let color = move || params.with(|params| params.get("color").cloned().unwrap_or_default());
    let name = move || params.with(|params| params.get("name").cloned().unwrap_or_default());

    view!{
        <div class="flex justify-center">
            <div class="mr-40">
                <ResultCard2 color={color()} text={name()} />
            </div>

            <div class="uppercase text-white font-bold my-auto">
                <div class="text-yellow-300">
                    <h2 class="text-3xl">
                        "winner"
                    </h2>
                    <p class="text-sm font-medium">
                        {round}
                    </p>
                    <p class="text-sm font-normal">
                        {sub_round}
                    </p>
                </div>

                <div class="my-14">
                </div>
                
                <p class="text-4xl">
                    "Congratulation !"
                </p>

                <div class="my-14">
                </div>

                <Button1 text="Continue" url="#"/>
            </div>
        </div>
    }
}
