mod site;
mod submod;

pub fn in_lib() {
    println!("Hello, world! lib.rs");
    site::Site::in_site_impl();
    submod::in_submod();
}
