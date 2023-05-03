use yew::{ html, Html, Component };
use crate::{ components::{ footer::Footer, menu::Menu, widget::Widget, project::ProjectItem }, router::Route };
use serde::{ Deserialize };
use ron::from_str;
use gloo_net::{http::Request, Error};

enum ComponentLoadingStage {
    Loading,
    Success,
    Error
}

pub struct Projects {
    products: Vec<Product>,
    loading_condition: ComponentLoadingStage
}

impl Component for Projects {
    type Message = Result<String, Error>;

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            products: Vec::new(),
            loading_condition: ComponentLoadingStage::Loading
        }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        let widgets = match self.loading_condition {
            ComponentLoadingStage::Loading => html! { <p>{"Loading assets."}</p> },
            ComponentLoadingStage::Success => {
                self.products.iter().map(|product| {
                    html! { <ProjectItem link={product.link.to_owned()} name={product.name.to_owned()} image_url={product.image_url.to_owned()} description={product.description.to_owned()} /> }
                }).collect::<Html>()
            },
            ComponentLoadingStage::Error => html! { <p>{ "There was an error loading the lesson plans!" }</p>}
        };
        html!{
            <div class="container-sm">
                <Menu names={ vec![ "O mnie".to_owned(), "Moje projekty".to_owned() ] } urls={ vec![ Route::Home, Route::Projects ] } />
                <Widget>
                    {widgets}
                </Widget>
                <Footer />
            </div>
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Ok( products ) => {
                self.products = from_str::<Vec<Product>>( &products ).unwrap().clone();
                self.loading_condition = ComponentLoadingStage::Success;
            },
            Err( _ ) => {
                self.loading_condition = ComponentLoadingStage::Error;
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &yew::Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, first_render: bool) {
        if first_render {
            let link = ctx.link().clone();
            wasm_bindgen_futures::spawn_local( async move {
                let fetch_products = Request::get( "./assets/products.ron" ).send().await.unwrap().text().await;
                link.send_message( fetch_products );
            } );
        }
    }

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, _ctx: &yew::Context<Self>) {}
}

#[derive(PartialEq, Deserialize, Clone)]
struct Product {
    pub name: String,
    pub image_url: String,
    pub description: String,
    pub link: String
}