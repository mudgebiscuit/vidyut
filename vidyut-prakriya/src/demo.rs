//! WASM-exported demo functions.
use crate::dhatupatha::Dhatupatha;
use crate::prakriya::{Prakriya, Rule, Step};
use serde::Serialize;

use crate::{args, Ashtadhyayi};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

/// A lightweight `Step` that exposes fewer private fields.
#[derive(Serialize)]
pub struct WebStep {
    rule: Rule,
    result: String,
}

/// A lightweight `Prakriya` that exposes fewer private fields.
#[derive(Serialize)]
pub struct WebPrakriya {
    text: String,
    history: Vec<WebStep>,
}

fn to_web_history(history: &[Step]) -> Vec<WebStep> {
    history
        .iter()
        .map(|x| WebStep {
            rule: x.rule(),
            result: x.result().clone(),
        })
        .collect()
}

fn to_web_prakriyas(prakriyas: &[Prakriya]) -> Vec<WebPrakriya> {
    prakriyas
        .iter()
        .map(|p| WebPrakriya {
            text: String::from(p.text()),
            history: to_web_history(p.history()),
        })
        .collect()
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn error(s: &str, value: &str);

    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn error_num(s: &str, value: usize);
}

/// WebAssembly API for vidyut-prakriya.
#[wasm_bindgen]
pub struct Vidyut {
    dhatupatha: Dhatupatha,
}

#[wasm_bindgen]
impl Vidyut {
    /// Creates a new API manager.
    ///
    /// This constructor is not called `new` because `new` is a reserved word in JavaScript.
    pub fn init(dhatupatha: &str) -> Self {
        Vidyut {
            dhatupatha: Dhatupatha::from_text(dhatupatha).unwrap(),
        }
    }

    /// Returns each possible *prakriyā* for the given args.
    ///
    /// TODO: how might we reduce the number of arguments here?
    #[allow(clippy::too_many_arguments)]
    pub fn derive(
        &self,
        code: &str,
        lakara: args::Lakara,
        prayoga: args::Prayoga,
        purusha: args::Purusha,
        vacana: args::Vacana,
        pada: Option<args::Pada>,
        sanadi: Option<args::Sanadi>,
    ) -> JsValue {
        use args::*;

        if let Some(dhatu) = self.dhatupatha.get(code) {
            let mut args = TinantaArgs::builder()
                .lakara(lakara)
                .prayoga(prayoga)
                .purusha(purusha)
                .vacana(vacana);

            if let Some(pada) = pada {
                args = args.pada(pada);
            }

            let args = args.build().unwrap();

            let a = Ashtadhyayi::new();
            let prakriyas = match sanadi {
                Some(s) => {
                    let dhatu = dhatu.with_sanadi(s);
                    a.derive_tinantas(&dhatu, &args)
                }
                None => a.derive_tinantas(dhatu, &args),
            };

            let web_prakriyas = to_web_prakriyas(&prakriyas);
            serde_wasm_bindgen::to_value(&web_prakriyas).unwrap()
        } else {
            error("[vidyut] Code not found:", code);
            serde_wasm_bindgen::to_value(&Vec::<WebPrakriya>::new()).unwrap()
        }
    }
}
