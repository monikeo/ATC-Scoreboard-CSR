use leptos::*;


#[component]
pub fn Border() -> impl IntoView {
    view!{
        <div>
            <svg
    width="303"
    height="102"
    viewBox="0 0 303 102"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
    preserveAspectRatio="none"
  >
    <path
      opacity="0.8"
      d="M103 1H9C4.58172 1 1 4.58172 1 9V93C1 97.4183 4.58172 101 9 101H294C298.418 101 302 97.4183 302 93V9C302 4.58172 298.418 1 294 1H197.5"
      stroke="#FFDF00"
    ></path>
  </svg>
        </div>
    }
}

#[component]
pub fn NumberBoxBg() -> impl IntoView {
    view!{
        <div class="w-[100px] h-20 absolute left-[9px] top-[11px] rounded-lg bg-[#ffdf00]/25"></div>
  <div class="w-[100px] h-20 absolute left-[190px] top-[11px] rounded-lg bg-[#ffdf00]/25"></div>
    }
}

#[component]
pub fn TwoCircle() -> impl IntoView {
    view!{
        <svg
      width="12"
      height="47"
      viewBox="0 0 12 47"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      //class="absolute w-3 h-[47px] left-[30px]"
      preserveAspectRatio="none"
    >
      <circle cx="6" cy="6" r="6" fill="#FFDF00" fill-opacity="0.7"></circle>
      <circle cx="6" cy="41" r="6" fill="#FFDF00" fill-opacity="0.7"></circle>
    </svg>
    }
}

