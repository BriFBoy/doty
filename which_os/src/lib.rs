use crate::{
    os_release::pars_os_release,
    ostypes::{Os, pars_os_type},
};

pub mod os_release;
pub mod ostypes;

pub fn get_os_type() -> Option<Os> {
    let map = pars_os_release().ok()?;
    let os_string = map.get("ID")?;

    Some(pars_os_type(os_string))
}
