use proc_macro::TokenStream;
use crate::parser::*;

const IMPL_CODE: &str = r#"
#[automatically_derived]
impl Hashable for [STRUCT-NAME] {
    fn update_hasher(&self, hasher: &mut impl sha2::Digest)  {
        [FIELDS-CODE]
    }
}"#;

const FIELD_CODE: &str = "self.[FIELD-NAME].update_hasher(hasher);\n";

pub fn derive_hashable(struct_desc: StructDesc) -> TokenStream {
    let mut fields_code = String::new();
    for field in struct_desc.fields {
        let field_code = FIELD_CODE.replace("[FIELD-NAME]", &field.name);
        fields_code.push_str(&field_code);
    }

    let code = IMPL_CODE
        .replace("[STRUCT-NAME]", &struct_desc.name)
        .replace("[FIELDS-CODE]", &fields_code);
    
    code.parse().unwrap()
}
