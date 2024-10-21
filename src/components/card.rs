use leptos::*;

#[component]
pub fn Card1(color: &'static str, text:String) -> impl IntoView {
    // #65121a dark red
    // #dd3e34
    // rgba(59,85,159,0.74) blue
    // #3b559f blue
    let custom_style = if color == "blue" {"box-shadow: 0px 0px 22px 4px rgba(59,85,159,0.74);"} else {"box-shadow: 0px 0px 22px 4px #65121a"};

    let flag = "bg-red-500 bg-blue-500";

    let class1 = format!("text-white p-8 rounded-lg flex flex-col items-center shadow-lg {}", if color == "red" {"bg-[#65121a]"} else {"bg-[#3b559f]"});

    let class2 = format!("bg-{}-500 p-8 rounded-full mb-4", color);

    view!{
        <div class={class1} style={custom_style}>
            <div class={class2}>
                <img src="path-to-moni-logo" alt="Logo"/>
            </div>
            <h2 class="text-3xl font-bold text-center">
                {text}
            </h2>
        </div>
    }
}

#[component]
pub fn RedCard1(text: String) -> impl IntoView {
    view!{
        <div>
            <Card1 color="red" text={text} />
        </div>
    }
}

#[component] 
pub fn BlueCard1(text: String) -> impl IntoView {
    view!{
        <div>
            <Card1 color="blue" text={text} />
        </div>
    }
}

#[component]
pub fn Card2(text: String, color: String) -> impl IntoView {
    let custom_style = if color == "red" {"box-shadow: 0px 0px 22px 3px #65121a;"} else {"box-shadow: 0px 0px 22px 3px rgba(59,85,159,0.74);"};
    let flag = "border-red-500 border-blue-500";
    let class1 = format!("bg-black border border-{}-500 text-yellow-500 p-8 rounded-lg flex flex-col items-center shadow-cl", color);
    let class2 = format!("bg-black p-12 rounded-full mb-4 border-4 border-{}-500", color);
    view!{
        <div class="flex flex-col">
            <h2 class="text-xl font-semibold uppercase mb-6 text-yellow-300">
                {text}
            </h2>

            <div class="border border-white rounded-sm w-52 h-80" style={custom_style}>
                <div>
                    <img src="path-to-moni-logo" alt="MONI Logo" />
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn RedCard2(text: String) -> impl IntoView {
    let color = "red".to_string();
    view!{
        <div>
            <Card2 text={text} color={color} />
        </div>
    }
}

#[component]
pub fn BlueCard2(text: String) -> impl IntoView {
    let color = "blue".to_string();
    view!{
        <div>
            <Card2 text={text} color={color} />
        </div>
    }
}

#[component]
pub fn ResultCard2(color: String, text: String ) -> impl IntoView {
    view!{
        <div>
            <Card2 text={text} color={color} />
        </div>
    }
}
