#[macro_use] extern crate nickel;

extern crate chrono;
extern crate postgres;
extern crate postgres_array;
extern crate rand;
extern crate r2d2;
extern crate nickel_postgres;
extern crate nickel_session as session;
extern crate nickel_cookies as cookies;

use std::collections::HashMap;
use std::env;
use r2d2::NopErrorHandler;
use postgres::SslMode;
use nickel::{Nickel, HttpRouter};
use nickel_postgres::{PostgresMiddleware, PostgresRequestExtensions};

//mod home;


/// Tässä tiedostossa
///
/// Luodaan serveri, käynnistetään se ja luodaan postgres middleware.
/// 
///
/// Koodityyli
/// ```
/// extern crate foo
///
/// use foo::bar;
/// use foo::bar::baz;
/// use std::thread;
///
/// fn funktio_alaviivalla(kuvaava_nimi_alaviivalla: LuokkaCamelCasella)->LuokkaCamelCasella<T>
/// {
///     let handle = thread::spawn({
///         "closuressa palautetaan bindaamaton merkkijono näin"
/// 
///     });
///     let kuvaava_nimi: LuokkaCamelCasella<T> = LuokkaCamelCasella::new();
///     let kuvaava_nimi2 = String::new();
///     return kuvaava_nimi;
/// }
/// fn hash(syote: String)->String
/// {
///     syote   // jos palautus funtion ainoa lause, palauta näin
/// }
///```
///Muista dokumentoida ja kommentoida. Kirjoita koodia jonka osaat selittää tai tee pull request
///vaikka olisit ylläpitäjä. Muuten dokumentoitu toimiva hyödyllinen koodi hyväksytään aina.
///Jos hyväksyntää (tai laiskanläksyä) ei kuulu ota yhteyttä. println!/print! debuggaus ei ole 
///toivottavaa, niitä on turha pullata. Testi koodille on toivottavaa, ei pakollista, pakollisuus
///sattuisi eniten allekirjoittaneeseen. 
///
///Purkasta. Nickel postgres, cookie ja session on huonosti (ei dokumentoitu tai kommentoitu) 
///kirjoitettuja. Purkaa tulee niistä ainakin aluksi, yritän kirjoittaa niille käyttöohjeet 
///englanniksi.

fn main(){
    let mut serveri = Nickel::new(); //Luodaan Serveri.
    //postgres on huonosti dokumentoitu, purkalla on jotain tekemistä yhteyksien määrällä.
    let osoite = "postgres://postgres@localhost/silmukka".to_string(); 
    let db = PostgresMiddleware::new(&osoite, 
                                     SslMode::None,
                                     5, //Purkka, nickel_postgres huonosti dokumentoitu
                                     Box::new(NopErrorHandler)).unwrap();*/
    serveri.utilize(db);
    let mut routers: Vec<nickel::router::router::Router> = Vec::new();
    //Luodaan routerit
    routers.push(home::route()); //"/"
    for router in routers
    {
        serveri.utilize(router);
    }
    serveri.listen("127.0.0.1:6767");
}
