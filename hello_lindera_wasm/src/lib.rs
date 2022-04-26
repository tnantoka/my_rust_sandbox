use lindera::tokenizer::Tokenizer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn tokenize(text: &str) -> JsValue {
    let tokenizer = Tokenizer::new().unwrap();
    let tokens = tokenizer.tokenize(text).unwrap();
    JsValue::from_serde(&tokens).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
