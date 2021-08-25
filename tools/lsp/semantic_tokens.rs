use lsp_types::{
    SemanticToken, SemanticTokenModifier, SemanticTokenType, SemanticTokens, SemanticTokensResult,
};
use sixtyfps_compilerlib::parser::SyntaxKind;

use crate::DocumentCache;

/// Give all the used types/modifier a number in an indexed array
macro_rules! declare_legend {
    ($what:ident: $ty:ty = [$($tk:ident)*]) => {
        pub const $what : &[$ty] = &[
            $(<$ty>::$tk),*
        ];
        declare_legend!{@  [ $($tk)* ], 0}
    };
    (@ [$head:ident $($tail:ident)*], $n:expr) => {
        const $head: u32 = $n;
        declare_legend!{@  [ $($tail)* ], $n + 1}
    };
    (@ [], $n:expr) => {};
}
// the id of the element
declare_legend!(LEGEND_TYPES : SemanticTokenType = [TYPE PARAMETER VARIABLE PROPERTY FUNCTION MACRO KEYWORD COMMENT STRING NUMBER OPERATOR]);
declare_legend!(LEGEND_MODS: SemanticTokenModifier = [DEFINITION DECLARATION]);

pub fn get_semantic_tokens(
    document_cache: &mut DocumentCache,
    text_document: &lsp_types::TextDocumentIdentifier,
) -> Option<SemanticTokensResult> {
    let uri = &text_document.uri;
    let filepath = uri.to_file_path().ok()?;
    let doc = document_cache.documents.get_document(&filepath)?;
    let doc_node = doc.node.as_ref()?;
    let mut token = doc_node.first_token()?;
    let mut data = vec![];
    let mut delta_start = 0;
    let mut delta_line = 0;
    loop {
        let t_m = match token.kind() {
            SyntaxKind::Comment => Some((self::COMMENT, 0)),
            SyntaxKind::StringLiteral => Some((self::STRING, 0)),
            SyntaxKind::NumberLiteral => Some((self::NUMBER, 0)),
            SyntaxKind::ColorLiteral => Some((self::NUMBER, 0)),
            SyntaxKind::Identifier => match token.parent().kind() {
                SyntaxKind::Component => Some((self::KEYWORD, 0)),
                // the id of the element
                SyntaxKind::SubElement => Some((self::VARIABLE, 1 << self::DEFINITION)),
                SyntaxKind::RepeatedElement => Some((self::KEYWORD, 0)),
                SyntaxKind::RepeatedIndex => Some((self::VARIABLE, 1 << self::DEFINITION)),
                SyntaxKind::ConditionalElement => Some((self::KEYWORD, 0)),
                SyntaxKind::CallbackDeclaration => Some((self::KEYWORD, 0)),
                SyntaxKind::CallbackConnection => Some((self::FUNCTION, 0)),
                SyntaxKind::PropertyDeclaration => Some((self::KEYWORD, 0)),
                SyntaxKind::PropertyAnimation => Some((self::KEYWORD, 0)),
                SyntaxKind::QualifiedName => match token.parent().parent().map(|p| p.kind()) {
                    Some(SyntaxKind::Type) => Some((self::TYPE, 0)),
                    // the base type
                    Some(SyntaxKind::Element) => Some((self::TYPE, 0)),
                    // FIXME: we should do actual lookup
                    Some(SyntaxKind::Expression) => None,
                    Some(SyntaxKind::StatePropertyChange) => Some((self::PROPERTY, 0)),
                    Some(SyntaxKind::PropertyAnimation) => Some((self::PROPERTY, 0)),
                    _ => None,
                },
                SyntaxKind::DeclaredIdentifier => match token.parent().parent().map(|p| p.kind()) {
                    Some(SyntaxKind::Component) => Some((self::TYPE, 1 << self::DEFINITION)),
                    Some(SyntaxKind::RepeatedElement) => {
                        Some((self::PROPERTY, 1 << self::DEFINITION))
                    }
                    Some(SyntaxKind::CallbackDeclaration) => {
                        Some((self::FUNCTION, 1 << self::DEFINITION))
                    }
                    Some(SyntaxKind::CallbackConnection) => {
                        Some((self::PARAMETER, 1 << self::DEFINITION))
                    }
                    Some(SyntaxKind::PropertyDeclaration) => {
                        Some((self::PROPERTY, 1 << self::DEFINITION))
                    }
                    Some(SyntaxKind::State | SyntaxKind::Transition) => {
                        // This is the state name, but what semantic type is that?
                        None
                    }
                    Some(SyntaxKind::StructDeclaration) => {
                        Some((self::TYPE, 1 << self::DEFINITION))
                    }
                    _ => None,
                },
                SyntaxKind::ChildrenPlaceholder => Some((self::MACRO, 0)),
                SyntaxKind::Binding | SyntaxKind::TwoWayBinding => Some((self::PROPERTY, 0)),
                SyntaxKind::ReturnStatement => Some((self::KEYWORD, 0)),
                SyntaxKind::AtImageUrl => Some((self::MACRO, 0)),
                SyntaxKind::AtLinearGradient => Some((self::MACRO, 0)),
                SyntaxKind::ConditionalExpression => Some((self::KEYWORD, 0)),
                SyntaxKind::ObjectMember => Some((self::PROPERTY, 1 << self::DECLARATION)),
                SyntaxKind::States => Some((self::KEYWORD, 0)),
                SyntaxKind::State => Some((self::KEYWORD, 0)),
                SyntaxKind::Transitions => Some((self::KEYWORD, 0)),
                SyntaxKind::Transition => Some((self::KEYWORD, 0)),
                SyntaxKind::ExportsList => Some((self::KEYWORD, 0)),
                SyntaxKind::ExportSpecifier => Some((self::KEYWORD, 0)),
                SyntaxKind::ExportIdentifier => Some((
                    self::TYPE,
                    if token.parent().parent().map_or(false, |p| {
                        p.children().find(|n| n.kind() == SyntaxKind::ExportName).is_some()
                    }) {
                        0
                    } else {
                        1 << self::DECLARATION
                    },
                )),
                SyntaxKind::ExportName => Some((self::TYPE, 1 << self::DECLARATION)),
                SyntaxKind::ImportSpecifier => Some((self::KEYWORD, 0)),
                SyntaxKind::ImportIdentifier => Some((self::KEYWORD, 0)),
                SyntaxKind::ExternalName => Some((
                    self::TYPE,
                    if token.parent().parent().map_or(false, |p| {
                        p.children().find(|n| n.kind() == SyntaxKind::InternalName).is_some()
                    }) {
                        0
                    } else {
                        1 << self::DECLARATION
                    },
                )),
                SyntaxKind::InternalName => Some((self::TYPE, 1 << self::DECLARATION)),
                SyntaxKind::ObjectTypeMember => Some((self::PROPERTY, 1 << self::DEFINITION)),
                SyntaxKind::StructDeclaration => Some((self::KEYWORD, 0)),
                _ => None,
            },
            SyntaxKind::PlusEqual
            | SyntaxKind::MinusEqual
            | SyntaxKind::StarEqual
            | SyntaxKind::DivEqual
            | SyntaxKind::LessEqual
            | SyntaxKind::GreaterEqual
            | SyntaxKind::EqualEqual
            | SyntaxKind::NotEqual
            | SyntaxKind::OrOr
            | SyntaxKind::AndAnd => Some((self::OPERATOR, 0)),
            SyntaxKind::LAngle | SyntaxKind::RAngle => (token.parent().kind()
                == SyntaxKind::PropertyDeclaration)
                .then(|| (self::OPERATOR, 0)),
            SyntaxKind::Plus
            | SyntaxKind::Minus
            | SyntaxKind::Star
            | SyntaxKind::Div
            | SyntaxKind::Equal => Some((self::OPERATOR, 0)),
            SyntaxKind::Question => Some((self::OPERATOR, 0)),
            SyntaxKind::At => Some((self::MACRO, 0)),
            _ => None,
        };
        if let Some((token_type, token_modifiers_bitset)) = t_m {
            data.push(SemanticToken {
                delta_line,
                delta_start,
                length: token.text().encode_utf16().count() as u32,
                token_type,
                token_modifiers_bitset,
            });
            delta_line = 0;
            delta_start = 0;
        }
        let text = token.text();
        let l = text.bytes().filter(|x| *x == b'\n').count();
        if l == 0 {
            delta_start += text.encode_utf16().count() as u32;
        } else {
            delta_line += l as u32;
            delta_start = text[(text.rfind('\n').unwrap() + 1)..].encode_utf16().count() as u32;
        }
        token = match token.next_token() {
            None => break,
            Some(token) => token,
        }
    }
    Some(SemanticTokensResult::Tokens(SemanticTokens { result_id: None, data }))
}
