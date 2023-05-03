use yew::{ html, Html, function_component };

#[function_component]
pub fn Footer() -> Html {
    html!{
        <footer class="row">
            <div class="col-sm-12 footer">
                <div class="row">
                    <div class="element-widget col-sm">
                        <p>{"Dane kontaktowe:"}</p>
                        <ul>
                            <li>{"tel. kom.: +48 608 673 146"}</li>
                            <li>{"e-mail: mateusz.krasuski.358@gmail.com"}</li>
                        </ul>
                    </div>
                    <div class="element-widget col-sm">
                        <p>{"Repozytoriom:"}</p>
                        <p><a href="https://github.com/Saurus42">{"https://github.com/Saurus42"}</a></p>
                    </div>
                </div>
            </div>
        </footer>
    }
}