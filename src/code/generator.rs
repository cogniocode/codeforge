use std::collections::HashMap;

use crate::code::generator::traditional::{TraditionalCodeGenerator, ParseOptionsError};
use crate::code::models::{Code, CodeKind, CodeLength};

pub trait CodeGenerator {
    fn generate(&self, len: CodeLength) -> Code;
}

pub fn from_code_kind(code_kind: CodeKind, options: HashMap<String, String>) -> Result<impl CodeGenerator, ParseOptionsError> {
    match code_kind {
        CodeKind::Traditional => TraditionalCodeGenerator::from_options(options),
        CodeKind::Memorable => Ok(TraditionalCodeGenerator::default())
    }
}

pub mod traditional {
    use std::collections::HashMap;
    use std::str::{FromStr, ParseBoolError};

    use rand::Rng;

    use crate::code::generator::CodeGenerator;
    use crate::code::models::{Code, CodeKind, CodeLength};
    use std::error::Error;
    use serde::__private::Formatter;

    const NUMBERS_POOL: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    const LETTERS_POOL: [char; 52] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    const SYMBOLS_POOL: [char; 5] = ['!', '(', ')', '?', '_'];

    pub struct TraditionalCodeGenerator {
        pub use_numbers: bool,
        pub use_symbols: bool,
    }

    #[derive(Debug)]
    pub struct ParseOptionsError;

    impl Error for ParseOptionsError {}

    impl std::fmt::Display for ParseOptionsError {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "Couldn't parse generator options.")
        }
    }

    impl From<ParseBoolError> for ParseOptionsError {
        fn from(_: ParseBoolError) -> Self {
            ParseOptionsError {}
        }
    }

    impl TraditionalCodeGenerator {
        pub fn from_options(options: HashMap<String, String>) -> Result<Self, ParseOptionsError> {
            let mut gen = Self::default();

            let parse_option = |option: &str| {
                options.get(option).map_or_else(|| None, |opt| Some(bool::from_str(opt)))
            };

            let use_numbers = parse_option("use_numbers").unwrap_or(Ok(true))?;
            let use_symbols = parse_option("use_symbols").unwrap_or(Ok(false))?;

            gen.use_numbers = use_numbers;
            gen.use_symbols = use_symbols;

            Ok(gen)
        }
    }

    impl Default for TraditionalCodeGenerator {
        fn default() -> Self {
            TraditionalCodeGenerator {
                use_numbers: true,
                use_symbols: false
            }
        }
    }

    impl CodeGenerator for TraditionalCodeGenerator {
        fn generate(&self, len: CodeLength) -> Code {
            let mut combined_pool = LETTERS_POOL.to_vec();

            if self.use_numbers {
                combined_pool.append(NUMBERS_POOL.to_vec().as_mut());
            }

            if self.use_symbols {
                combined_pool.append(SYMBOLS_POOL.to_vec().as_mut())
            }

            let mut rng = rand::thread_rng();
            let password: String = (0..len)
                .map(|_| {
                    let i = rng.gen_range(0..combined_pool.len());
                    combined_pool.get(i).unwrap()
                })
                .collect();

            Code {
                kind: CodeKind::Traditional,
                value: password,
                length: len
            }
        }
    }
}

