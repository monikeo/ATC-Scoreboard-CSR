use leptos::*;

#[component]
pub fn OuterContainer(children: Children) -> impl IntoView {
    view!{
        <div class="min-h-screen w-screen bg-black flex items-center justify-center">
            {children()}
        </div>
    }
}

#[component]
pub fn InnerContainer(children: Children) -> impl IntoView {
    view!{
        <div class="max-w-9/10 min-h-screen bg-black p-4 flex items-center justify-center">    
            {children()}
        </div>
    }
}
