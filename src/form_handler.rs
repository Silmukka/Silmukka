///Luodaan parseri formille, jotta purkka ei näy muualla
use std::collections::HashMap;
pub fn handle_form(_form: &mut str)->HashMap<String, String>
{
    let mut form_data = HashMap::new();
    let mut form = _form.to_string();
    loop{
        if form.is_empty() == true{
            break;
        }
        let mut temp_hash = ("".to_string(), "".to_string());
        let mut temp = String::new();
        for c in form.clone().chars(){
            if c == '&'
            {
                form.remove(0);
                break;
            }
            if c == '='
            {
                temp_hash.0 = temp;
                temp = "".to_string();
                form.remove(0);
            }
            else
            {
                temp.push(c);
                form.remove(0);
            }
        }
        temp_hash.1 = temp;
        form_data.insert(temp_hash.0, temp_hash.1);
        if form.is_empty() == true{
            break;
        }
    }
    return form_data;
 
}
