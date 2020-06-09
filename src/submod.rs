use super::site::Site;

pub(crate) fn in_submod() {
    println!("Hello, world! submod.rs");
    Site::in_site_impl();
}
