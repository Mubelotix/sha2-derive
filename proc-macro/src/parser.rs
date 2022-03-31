use proc_macro::*;

// In order to add a method to a struct we don't own
trait HackTrait {
    fn next_useful(&mut self) -> Option<TokenTree>;
}

impl HackTrait for proc_macro::token_stream::IntoIter {
    fn next_useful(&mut self) -> Option<TokenTree> {
        match self.next() {
            Some(TokenTree::Ident(ident)) if ident.to_string() == "pub" => {
                match self.next() {
                    Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => self.next_useful(),
                    t => t,
                }
            },
            Some(TokenTree::Punct(punct)) if punct.as_char() == '#' => {
                match self.next() {
                    Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Bracket => self.next_useful(),
                    t => t,
                }
            },
            t => t,
        }
    }
}

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

    // Struct
    match tokens.next_useful() {
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
        let field_name = match tokens.next_useful() {
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
