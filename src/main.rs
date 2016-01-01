#![feature(box_syntax, box_patterns)]

#[deny(warnings)]
#[macro_use] extern crate nickel;

extern crate chrono;
extern crate postgres;
extern crate postgres_array;
extern crate rand;
extern crate r2d2;
extern crate nickel_postgres;
extern crate nickel_session as session;
extern crate nickel_cookies as cookies;
extern crate crypto;
extern crate time;
//extern crate rustc_serialize;
//extern crate hyper;
extern crate md5;

use r2d2::NopErrorHandler;
use postgres::SslMode;
use nickel::Nickel;
use nickel_postgres::PostgresMiddleware;
use time::Duration;
use std::hash::{Hash, SipHasher, Hasher};

mod form_handler;
mod home;
mod login;
mod event;
mod user;

pub use user::parse; 
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
fn hashaus<T>(obj: T) -> u64
    where T: Hash
{
    let mut hasher = SipHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}
//Session ja sen key, ServerData on tarvittava
pub struct ServerData;
pub static SECRET_KEY: &'static cookies::SecretKey = &cookies::SecretKey([0; 32]);

impl cookies::KeyProvider for ServerData {
    fn key(&self) -> cookies::SecretKey { SECRET_KEY.clone() }
}

impl session::Store for ServerData {
    type Session = Option<String>;

    fn timeout() -> Duration {
            Duration::minutes(55)
        }
}

fn main(){
    let mut serveri = Nickel::with_data(ServerData); //Luodaan Serveri.
    //Nickel on huonosti dokumentoitu, purkalla on jotain tekemistä yhteyksien määrällä.
    let osoite = "postgres://postgres@localhost/silmukka".to_string(); 
    let db = PostgresMiddleware::new(&osoite, 
                                     SslMode::None,
                                     5, //Purkka, nickel_postgres huonosti dokumentoitu
                                     Box::new(NopErrorHandler)).unwrap();
    serveri.utilize(db);
    //Luodaan routet serverillä
    let mut routers = vec![home::route()]; //"/"
    routers.push(login::validation_router());
    routers.push(login::login_router());
    routers.push(event::create_event());
    routers.push(event::watcher());
    routers.push(event::creator());
    routers.push(user::register_router());
    routers.push(user::valid_router());
    routers.push(user::user_router());
    for router in routers{
        serveri.utilize(router);
    }

    serveri.listen("127.0.0.1:6767");
}
