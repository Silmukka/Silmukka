///Tässä tiedostossa luodaan routeri "/" polulle. Jos käyttäjä on kirjautuneena, näytetään omat
///tapahtumat, suosituimmat tapahtumat, lippukunnan tapahtumat ja hallinoimat tapahtumat. Jos ei ole, kerrotaan mikä on Silmukka ja ohjataan 
///rekisteröitymään
use nickel::{Nickel, HttpRouter};
use nickel::router::router::Router;
use session::{Session, CookieSession};
use std::collections::HashMap;
use ServerData;
pub fn route()->Router<ServerData>{
    let mut home: Router<ServerData> = Nickel::router();
    home.get("/", middleware!{|req, mut vastaus| 
        let mut palautetaan = HashMap::new();
        let mut logged = false;
        match *CookieSession::get_mut(req, &mut vastaus){
            Some(_) => logged = true,
                         _ => ()
        } 
        if logged == false{
            palautetaan.insert("kirjaudu", "Kirjaudu sisään");
            return vastaus.render("assets/out_index.html", &palautetaan);
        }
        palautetaan.insert("kirjaudu", "Kirjaudu ulos");
        return vastaus.render("assets/index.html", &palautetaan);
            
        
    });
    return home;
}
