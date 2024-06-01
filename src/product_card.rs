use leptos::*;

#[component]
pub fn ProductCard(name:String)->impl IntoView{
    view! {
        <div>
        {name}
        </div>
    }
}