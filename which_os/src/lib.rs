use crate::{
    os_release::pars_os_release,
    ostypes::{Os, pars_os_type},
};

pub mod os_release;
pub mod ostypes;

pub fn get_os_id() -> Option<String> {
    let map = pars_os_release().ok()?;
    let os_string = map.get("ID")?;

    Some(os_string.clone())
}
pub fn get_os_id_like() -> Option<Vec<String>> {
    let map = pars_os_release().ok()?;
    let os_string = map.get("ID_LIKE")?;
    let os_list: Vec<String> = os_string.split(" ").map(String::from).collect();

    Some(os_list)
}
pub fn get_os_type() -> Option<Os> {
    let map = pars_os_release().ok()?;
    let os_string = map.get("ID")?;

    Some(pars_os_type(os_string))
}
