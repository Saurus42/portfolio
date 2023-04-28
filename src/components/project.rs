use yew::{ html, Html, Properties, function_component };

#[function_component]
pub fn ProjectItem( props: &Props ) -> Html {
    html!{
        <div class="project">
            <img src={props.image_url.clone()} alt={props.name.clone()} />
            <p>{format!( "Project: {}", props.name )}</p>
            <p>{props.description.clone()}</p>
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub image_url: String,
    pub description: String
}