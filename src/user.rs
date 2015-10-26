///Tiedostossa on käyttäjien rekisteröityminen ja tarkastelu. Toivotaan integroimista Kuksaan
///vuonna 20X0
use nickel_postgres::PostgresRequestExtensions;
use session::{Session, CookieSession};
use nickel::*;
use std::io::Read;
use std::collections::HashMap;
use rand::{Rng, thread_rng};
use ServerData;
use form_handler;
use hashaus;
fn suolaus()->String
{
    let lista = "aababcdefghijklmnopqrstuvwxyzåäöABCDEFGHIJKLMNOPQRSTUVWXYZÅÄÖ1234567890!''<#¤%&/()=?
        @£$‰‚{[]}*^.,_-".to_string();
    let mut suola = String::new();
    let mut rng = thread_rng();
    for _ in 0..32
    {
        let a = rng.gen_range(1, lista.len());
        if a >= lista.len()
        {
            panic!("a oli isompi kuin lista");
        }
        let mut int = 0;
        for c in lista.chars()
        {
            if int == a
            {
                suola.push(c);
            }
            int+=1;
        }
    }
    return suola;
}
pub fn parse(parsittava: String)->String
{
    let mut parsittu = parsittava.replace("%40", "@");
    parsittu = parsittu.replace("+", " ");
    return parsittu;
}
pub fn register_router()->router::router::Router<ServerData>
{  
    let mut register: router::router::Router<ServerData> = Nickel::router();
    register.get("/register", middleware!{ |req, mut res| <ServerData>
        let logged = match *CookieSession::get_mut(req, &mut res)
        {
            Some(_) => true,
            _ => false
        };
        if logged == false
        {
            let mut data = HashMap::new();
            data.insert("kirjaudu", "Kirjaudu sisään");
            return res.render("assets/register_form.html", &data);
        }
        "<html><body><script>window.location.href='/'</script></body></html>"
    });
    return register;
}  
pub fn valid_router()->router::router::Router<ServerData>
{  
    let mut valid: router::router::Router<ServerData> = Nickel::router();
    valid.post("/valid_register", middleware!{|req, res|
        let conn = req.db_conn();
        let stmt = conn.prepare("SELECT id FROM kayttaja WHERE $1 = $2").unwrap();
        let mut form_data = String::new();
        let _ = req.origin.read_to_string(&mut form_data);
        let form = form_handler::handle_form(&mut form_data);
        let mut data = HashMap::new();
        for _ in stmt.query(&[&"kayttajanimi", &form.get("sposti")]).unwrap()
        {
            data.insert("virhe", "Käyttäjänimi varattu");
            return res.render("assets/register_form.html", &data);
        }
        for _ in stmt.query(&[&"sposti", &form.get("kayttajanimi")]).unwrap()
        {
            data.insert("virhe", "Sähköposti varattu");
            return res.render("assets/register_form.html", &data);  
        }
        if form.get("salasana").unwrap().len()<9
        {
            data.insert("virhe", "Liiaan lyhyt salasana. Salasanan minimipituus 9 merkkiä");
            return res.render("assets/register_form.html", &data); 
        }
        if form.get("salasana").unwrap() != form.get("salis").unwrap()
        {
            data.insert("virhe", "Erilaiset salasanat");
            return res.render("assets/register_form.html", &data); 
        }
        let suola = suolaus();
        let salasana = hashaus(suola.to_string()+form.get("salasana").unwrap());
        let ika = form.get("ika").unwrap();
        let lippukunta = form.get("lpk").unwrap();
        conn.execute("INSERT INTO kayttaja (kayttajanimi, suola, lippukunta, nimi, sposti, ika, 
            salasana) VALUES ($1, $2, $3, $4, $5, $6, $7)", &[&form.get("kayttaja").unwrap(),
                      &suola, &lippukunta, &form.get("nimi").unwrap(), 
                      &form.get("sposti").unwrap(), &ika, &(salasana as i64)]).unwrap();
        "<html><body><script>window.location.href='/login'</script></body></html>"
        
    });
    return valid;
}
pub fn user_router()->router::router::Router<ServerData>
{
    let mut valid: router::router::Router<ServerData> = Nickel::router();
    valid.get("/user/:username", middleware!{ | req, mut res|
        let logged = match *CookieSession::get_mut(req, &mut res)
        {
            Some(_) => true,
            _ => false
        };
        let conn = req.db_conn();
        let mut data = HashMap::new();
        print!("{}", logged);
        if logged == true
        {
            let stmt = conn.prepare("SELECT * FROM kayttaja WHERE kayttajanimi = $1").unwrap();
            for row in stmt.query(&[&req.param("username").unwrap()]).unwrap()
            {
                let lpk_url_get: String = parse(row.get("lippukunta"));
                let lpk_url = "/lpk/".to_string()+&lpk_url_get;
                data.insert("kayttajanimi", parse(row.get("kayttajanimi")));          
                data.insert("nimi",parse(row.get("nimi")));          
                data.insert("lpk",parse(row.get("lippukunta")));          
                data.insert("sposti", parse(row.get("sposti")));          
                data.insert("ika", row.get("ika"));
                data.insert("lpk_url", lpk_url);
            }
            return res.render("assets/user.html", &data);
        }
        "<html><body><script>window.location.href='/'</script></body></html>"
    });
    return valid;

}
