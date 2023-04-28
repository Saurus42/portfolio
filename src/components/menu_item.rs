use yew::{ html, Html, Properties, function_component };
use yew_router::prelude::Link;

use crate::router::Route;

#[function_component]
pub fn ItemMenu( props: &Props ) -> Html {
    html!{ <Link<Route> classes="item-navigator" to={props.url_link.clone()}>{props.name_link.clone()}</Link<Route>> }
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub name_link: String,
    pub url_link: Route
}