use yew::{ html, Html, function_component };
use crate::{ components::{ footer::Footer, menu::Menu, widget::Widget, project::ProjectItem }, router::Route };

#[function_component]
pub fn Projects() -> Html {
    let mut widgets = Vec::new();
    let content = vec![
        Product { name: "Skoki narciarskie 2020", link: "https://gameplanet.onet.pl/gry-online/sportowe/zimowe/skoki-narciarskie-2020/c19704m", image_url: "./images/skoki-narciarskie.jpg", description: "Gra przeglądarkowa bazująca na silniku Phaser 3 na potrzeby Onet.pl" },
        Product { name: "Hamster and Hammer", link: "https://play.google.com/store/apps/details?id=pl.hamsterentertainment.hamsterandhammer", image_url: "./images/hamster-and-hammer.jpg", description: "Własny projekt polegający na stworzeniu gry na telefon z systemem android wykorzystując Cordova i Phaser 3" },
        Product { name: "Praca dyplomowa", link: "https://github.com/Saurus42/One-of-Ten", image_url: "./images/praca-dyplomowa.jpg", description: "Praca dyplomowa wzorująca się na teleturnieju 10 z dziesięciu. Wszystko napisane w JavaScript korzystając z Node.js oraz Electron." },
        Product { name: "Bot szachowy", link: "https://github.com/Saurus42/chessarbiter", image_url: "./images/bot-szachowy.jpg", description: "Własny projekt polegający na stworzeniu bot szachowego na platformę discord." },
        Product { name: "Kalkulator do przelicznika walut", link: "https://github.com/Saurus42/bank-calculate", image_url: "./images/bank-calculate.jpg", description: "Prosta witryna napisana na potrzeby sprawdzenia moich umiejętności szybkiego uczenia się." },
        Product { name: "Moduł ułatwiający obsługę ciasteczek", link: "https://github.com/Saurus42/cookie", image_url: "./images/cookie.jpg", description: "Moduł skryptowy i binarny do obsługi ciasteczek po stronie klienta w wygodny sposób." }
    ];
    for element in content {
        widgets.push( html!{ <Widget><ProjectItem link={element.link.to_owned()} name={element.name.to_owned()} image_url={element.image_url.to_owned()} description={element.description.to_owned()} /></Widget> } )
    }
    html!{
        <div class="container">
            <Menu names={ vec![ "O mnie".to_owned(), "Moje projekty".to_owned() ] } urls={ vec![ Route::Home, Route::Projects ] } />
            {widgets}
            <Footer />
        </div>
    }
}

struct Product<'a> {
    pub name: &'a str,
    pub image_url: &'a str,
    pub description: &'a str,
    pub link: &'a str
}