use proc_macro::*;

#[derive(Debug)]
pub struct FieldDesc {
    pub name: String,
    pub ty: String,
}

#[derive(Debug)]
pub struct StructDesc {
    pub name: String,
    pub fields: Vec<FieldDesc>,
}

pub fn read_struct(tokens: TokenStream) -> StructDesc {
    let mut tokens = tokens.into_iter();

    match tokens.next() {
        Some(TokenTree::Ident(ident)) if ident.to_string() == "struct" => (),
        _ => panic!("Expected struct"),
    };

    let name = match tokens.next() {
        Some(TokenTree::Ident(ident)) => ident.to_string(),
        _ => panic!("Expected struct name"),
    };

    let struct_content = match tokens.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => group.stream(),
        _ => panic!("Expected brackets after struct name"),
    };
    tokens = struct_content.into_iter();

    let mut fields = Vec::new();
    loop {
        let field_name = match tokens.next() {
            Some(TokenTree::Ident(ident)) => ident.to_string(),
            None => break,
            _ => panic!("Expected field name"),
        };

        match tokens.next() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == ':' => (),
            _ => panic!("Expected colon after field name"),
        };

        let mut field_ty = String::new();
        loop {
            match tokens.next() {
                Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => break,
                Some(token) => field_ty.push_str(&token.to_string()),
                None => break,
            };
        }

        fields.push(FieldDesc {
            name: field_name,
            ty: field_ty,
        });
    }

    StructDesc {
        name,
        fields,
    }
}