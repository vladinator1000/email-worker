extern crate cfg_if;
extern crate handlebars;
extern crate reqwest;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use handlebars::Handlebars;
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

pub async fn fetch_template(template_name: &str) -> reqwest::Result<String> {
    let url = format!(
        "https://raw.githubusercontent.com/vladinator1000/email-worker/master/templates/{}.hbs",
        template_name
    );
    let response = reqwest::get(&url).await?.text().await?;

    Ok(format!("{:#?}", response))
}

#[wasm_bindgen]
pub async fn send(_to: String, _from: String, template_name: String, _data: String) -> String {
    let template = fetch_template(&template_name).await.unwrap();
    let handlebars = Handlebars::new();

    let data = ();
    handlebars.render_template(&template, &data).unwrap()
}
