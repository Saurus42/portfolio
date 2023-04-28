use yew::{ html, Html, Properties, Children, function_component };

#[function_component]
pub fn Widget( props: &Props ) -> Html {
    html!{
        <div class="widget">
            { for props.children.iter() }
        </div>
    }
}

#[derive( Clone, Properties, PartialEq )]
pub struct Props {
    pub children: Children
}