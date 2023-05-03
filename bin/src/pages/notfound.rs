use yew::{ Html, html, function_component };

use crate::{components::menu::Menu, router::Route};

#[function_component]
pub fn NotFound() -> Html {
    html!{ <div class="container-sm">
        <Menu names={ vec![ "O mnie".to_owned(), "Moje projekty".to_owned() ] } urls={ vec![ Route::Home, Route::Projects ] } />
        <p>{"Nie znaleziono takiej sekcji."}</p>
    </div> }
}