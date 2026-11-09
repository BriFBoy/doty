#[derive(PartialEq, Eq)]
pub enum Os {
    Arch,
    Fedora,
    Debian,
    Unknown,
}

pub(super) fn pars_os_type(os: &str) -> Os {
    match os {
        "arch" => Os::Arch,
        "fedora" => Os::Fedora,
        "debian" => Os::Debian,
        _ => Os::Unknown,
    }
}
