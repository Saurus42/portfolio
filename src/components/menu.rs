use yew::{ html, Html, Properties, function_component };
use crate::components::menu_item::ItemMenu;
use crate::router::Route;

#[function_component]
pub fn Menu( props: &Props ) -> Html {
    let mut names = Vec::new();
    let mut urls = Vec::new();
    for name in props.names.clone() {
        names.push( name );
    }
    for url in props.urls.clone() {
        urls.push( url );
    }
    let mut items = Vec::new();
    for index in 0..names.len() {
        items.push( html!{ <ItemMenu url_link={ urls[index].clone() } name_link={ names[index].clone() } /> } );
    }
    html!{
        <nav class="navigation">
            { items }
        </nav>
    }
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub names: Vec<String>,
    pub urls: Vec<Route>
}