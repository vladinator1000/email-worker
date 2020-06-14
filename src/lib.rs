extern crate cfg_if;
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

pub async fn fetch_template(_template_name: &str) -> reqwest::Result<String> {
    let response =
        reqwest::get("https://gist.github.com/vladinator1000/68dc63e30fa6397c8dcf4cabd619d2e0/raw/655eb4f6481a219e2fe29b7b4819ba6a3f2b43f0/signup_email_template.hbs")
            .await?
            .text()
            .await?;

    Ok(format!("{:#?}", response))
}

#[wasm_bindgen]
pub async fn send(_to: String, _from: String, template_name: String, _data: String) -> String {
    let template = fetch_template(&template_name).await.unwrap();
    let handlebars = Handlebars::new();

    let data = ();
    handlebars.render_template(&template, &data).unwrap()
}
