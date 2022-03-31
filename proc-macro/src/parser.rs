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

    // Visibility
    let next_token = match tokens.next() {
        Some(TokenTree::Ident(ident)) if ident.to_string() == "pub" => {
            match tokens.next() {
                Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => tokens.next(),
                t => t,
            }
        },
        t => t,
    };

    // Struct
    match next_token {
        Some(TokenTree::Ident(ident)) if ident.to_string() == "struct" => (),
        _ => panic!("Expected struct"),
    };

    // Name
    let name = match tokens.next() {
        Some(TokenTree::Ident(ident)) => ident.to_string(),
        _ => panic!("Expected struct name"),
    };

    // Brackets
    let struct_content = match tokens.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => group.stream(),
        _ => panic!("Expected brackets after struct name"),
    };
    tokens = struct_content.into_iter();

    // Fields
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
        let mut opened_brackets = 0;
        loop {
            match tokens.next() {
                Some(TokenTree::Punct(punct)) if punct.as_char() == ',' && opened_brackets == 0 => break,
                Some(TokenTree::Punct(punct)) if punct.as_char() == '<' => opened_brackets += 1,
                Some(TokenTree::Punct(punct)) if punct.as_char() == '>' => opened_brackets -= 1,
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
