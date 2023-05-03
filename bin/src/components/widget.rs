use yew::{ html, Html, Properties, Children, function_component };

#[function_component]
pub fn Widget( props: &Props ) -> Html {
    html!{
        <div class="row">
            <div class="col-sm-12 widget">
                <div class="row">
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}

#[derive( Clone, Properties, PartialEq )]
pub struct Props {
    pub children: Children
}