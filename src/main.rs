#[macro_use] extern crate nickel;

extern crate chrono;
extern crate postgres;
extern crate postgres_array;
extern crate rand;

use std::env;
use r2d2::NopErrorHandler;
use postgres::SslMode;
use nickel::{Nickel, HttpRouter};
use nickel_postgres::{PostgresMiddleware, PostgresRequestExtensions};

mod database;
mod router;

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
///sattuisi eniten testaajaan.

fn main(){

}
