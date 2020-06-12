extern crate cfg_if;
extern crate handlebars;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use handlebars::{
    to_json, Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError,
};
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn send() -> String {
    // let mut handlebars = Handlebars::new();

    let name = "signup_venue";
    // // register template from a file and assign a name to it
    // handlebars
    //     .register_template_file(name, "../templates/signup_venue.hbs")
    //     .unwrap();

    // let data = ();
    // handlebars.render(name, &data).unwrap();
    name.to_string()
}
