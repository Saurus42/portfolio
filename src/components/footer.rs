use yew::{ html, Html, function_component };
use crate::components::widget::Widget;

#[function_component]
pub fn Footer() -> Html {
    html!{
        <footer>
            <Widget>
                <div class="element-widget">
                    <p>{"Dane kontaktowe:"}</p>
                    <ul>
                        <li>{"tel. kom.: +48 608 673 146"}</li>
                        <li>{"e-mail: mateusz.krasuski.358@gmail.com"}</li>
                    </ul>
                </div>
                <div class="element-widget">
                    <p>{"Repozytoriom:"}</p>
                    <p><a href="https://github.com/Saurus42">{"https://github.com/Saurus42"}</a></p>
                </div>
            </Widget>
        </footer>
    }
}