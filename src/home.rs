///Tässä tiedostossa luodaan routeri "/" polulle. Jos käyttäjä on kirjautuneena, näytetään omat
///tapahtumat, suosituimmat tapahtumat, lippukunnan tapahtumat ja hallinoimat tapahtumat. Jos ei ole, kerrotaan mikä on Silmukka ja ohjataan 
///rekisteröitymään
use nickel::{Nickel, HttpRouter};
use nickel::router::router::Router;
use std::collections::HashMap;
pub fn route()->Router{
    let mut home = Nickel::router();
    home.get("/", middleware!{|_, vastaus|
        let mut palautetaan = HashMap::new();
        palautetaan.insert("kirjaudu", "Kirjaudu");
        return vastaus.render("assets/index.html", &palautetaan);
    
    });
    return home;
}
