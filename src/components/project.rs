use yew::{ html, Html, Properties, function_component };

#[function_component]
pub fn ProjectItem( props: &Props ) -> Html {
    html!{
        <a href={props.link.clone()} class="project col-sm-3">
            <article class="row">
                <header>
                    <img src={props.image_url.clone()} class="col-sm-12" alt={props.name.clone()} />
                </header>
                <section>
                    <p>{format!( "Projekt: {}", props.name )}</p>
                    <p>{props.description.clone()}</p>
                </section>
            </article>
        </a>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub image_url: String,
    pub description: String,
    pub link: String
}