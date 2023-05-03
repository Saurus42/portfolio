mod router;
mod pages;
mod components;

use pages::{home::Home, notfound::NotFound, projects::Projects};
use router::Route;
use wasm_bindgen::prelude::*;
use yew::{prelude::*, Renderer};
use yew_router::{ BrowserRouter, Switch };

fn switch( route: Route ) -> Html {
    match route {
        Route::Home => html!{ <Home /> },
        Route::Projects => html!{ <Projects /> },
        _ => html!{ <NotFound /> }
    }
}

#[function_component(Main)]
pub fn app() -> Html {
    html!{
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}


#[wasm_bindgen(start)]
fn run() {
    Renderer::<Main>::new().render();
}