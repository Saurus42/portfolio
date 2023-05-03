use yew::{ html, Html, function_component };
use yew_router::prelude::Link;
use crate::{ components::{ menu::Menu, footer::Footer }, router::Route };

#[function_component]
pub fn Home() -> Html {
    html!{
        <div class="container-sm">
            <Menu names={ vec![ "O mnie".to_owned(), "Moje projekty".to_owned() ] } urls={ vec![ Route::Home, Route::Projects ] } />
            <div class="row">
                <div class="col-sm-12 content">
                    <p>{"Nazywam się Mateusz Krasuski"}</p>
                    <p>{"Specjalizuję się w językach takich jak JavaScript/TypeScript do tego uczę się języka Rust."}</p>
                    <p>{"Stosowane technologie dla JavaScript/TypeScript to:"}</p>
                    <ul>
                        <li>{"React.js"}</li>
                        <li>{"Node.js"}</li>
                        <li>{"Koa.js"}</li>
                        <li>{"Express.js"}</li>
                        <li>{"Vue.js"}</li>
                    </ul>
                    <p>{"Znam również języki takie jak:"}</p>
                    <ul>
                        <li>{"PHP"}</li>
                        <li>{"C"}</li>
                        <li>{"C++"}</li>
                        <li>{"C#"}</li>
                        <li>{"Podstawy Java"}</li>
                    </ul>
                    <p>{"Zajmuję się frontend-em jak i backend-em."}</p>
                    <p>
                        {"Wszystkie moje projekty są do wglądu w "}
                        <a href="https://github.com/Saurus42">{"moim repozytorium"}</a>
                        {" oraz w zakładce "}
                        <Link<Route> to={Route::Projects}>{"Moje projekty"}</Link<Route>>
                        {"."}
                    </p>
                </div>
            </div>
            <Footer />
        </div>
    }
}