use leptos::*;
use leptos_router::*;

use crate::{
    components::{
        container::{
            InnerContainer
        },
        button::{
            Button1,
            Button3,
        },
        card::{
            BlueCard1,
            RedCard1
        }
    }
};

#[component]
pub fn VsPage() -> impl IntoView {
    let params = use_params_map();
    let round = move || params.with(|params| params.get("round").cloned().unwrap_or_default());
    let subround = move || params.with(|params| params.get("subround").cloned().unwrap_or_default());
    let red = move || params.with(|params| params.get("red").cloned().unwrap_or_default());
    let blue = move || params.with(|params| params.get("blue").cloned().unwrap_or_default());
    let route = format!("/battle_timer/{}/{}/{}/{}", round(), subround(), red(), blue());
    view!{
        <InnerContainer>
            <div class="flex flex-col items-center space-y-20">
                <div class="flex space-x-20">
                    <RedCard1 text={red()}/>
                    <div class="text-white text-4xl font-bold self-center">
                        "V.S"
                    </div>
                    <BlueCard1 text={blue()}/>
                </div>

                <div class="flex space-x-44">
                    <Button1 text="Menu" url="/" />
                    <Button3 text="Start" url={route} />
                    <Button1 text="Reset" url="#" />
                </div>
            </div>

        </InnerContainer>
    }
}

