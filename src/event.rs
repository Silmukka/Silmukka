use nickel_postgres::PostgresRequestExtensions;
use session::{Session, CookieSession};
use nickel::*;
use std::collections::HashMap;
use std::io::Read;
use ServerData;
use form_handler;
pub type Reititin = router::router::Router<ServerData>;
pub fn create_event()->Reititin
{
    let mut creator: router::router::Router<ServerData> = Nickel::router();
    creator.get("/create_event", middleware!{ |req, mut res| <ServerData>
        let conn = req.db_conn();
        let stmt = conn.prepare("select ika from kayttaja where kayttajanimi = $1").unwrap();
        let a = "Foul play".to_string();
        let mut man = String::new();
        assert_eq!(man, String::new());
            {man= match *CookieSession::get_mut(req, &mut res){
                Some(ref name)=>name,             
                _ => &a
                }.to_string();
        }
        println!("{}", man);
        for row in stmt.query(&[&man]).unwrap()
        {
            let int: i32 = row.get(0);
            if int < 4{
                panic!("at the disko");
            }
        }
        let mut data = HashMap::new();
        data.insert("Kirjaudu", "Kirjaudu ulos");
        return res.render("assets/creator.html", &data);

    });
    return creator;
}
pub fn creator()->Reititin
{
    let mut creator: Reititin = Nickel::router();
    creator.post("/event_create", middleware!{|req, res| <ServerData>
        let conn = req.db_conn();
        let mut form_data = String::new();
        req.origin.read_to_string(&mut form_data).unwrap();
        
        let form = form_handler::handle_form(&mut form_data);
        conn.execute("INSERT INTO tapahtuma (nimi, kuvaus) VALUES($1, $2)",
        &[form.get("nimi").unwrap(), form.get("kuvaus").unwrap()]).unwrap(); 
        "<html><body><script>document.location.href = '/event/".to_string()+&form.get("nimi").unwrap()+"'</script>
        </body></html>"
    });
    return creator;
}
pub fn watcher()->Reititin
{
    let mut watcher: Reititin = Nickel::router();
    watcher.get("/event/:event_name", middleware!{| req, mut res| <ServerData>
        let conn = req.db_conn();
        let stmt = conn.prepare("SELECT * FROM tapahtuma WHERE nimi = $1").unwrap();
        let empty = String::new();
        let stmt_id = conn.prepare("SELECT id FROM kayttaja WHERE kayttajanimi = $1").unwrap();
        let mut guy = String::new();
        assert_eq!(guy, String::new());
        {guy= match *CookieSession::get_mut(req, &mut res)
            {
                Some(ref name)=>name,
                _ => &empty
            
            }.to_string();
        }
        let mut id: i32 = 0;
        for row in stmt_id.query(&[&guy]).unwrap()
        {
            id = row.get(0);
        }
        if id != 0
        {
            print!("");
        }
        let mut data = HashMap::new();
        data.insert("Kirjaudu".to_string(), "Kirjaudu ulos".to_string());
        for row in stmt.query(&[&req.param("event_name").unwrap()]).unwrap()
        {
            data.insert("nimi".to_string(), row.get(1));
            data.insert("kuvaus".to_string(), row.get(2));
        }
        return res.render("assets/event.html", &data);
    });
    return watcher;
}
