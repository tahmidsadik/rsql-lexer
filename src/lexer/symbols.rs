use std::option::Option;
use std::string::String;

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Select,
    Create,
    Insert,
    Update,
    Delete,
    Drop,
    From,
    Into,
    Where,
    Distinct,
    Alter,
    Add,
    Set,
    Truncate,
    As,
    Asc,
    Desc,
    Between,
    Having,
    In,
    Join,
    Exists,
    Like,
    Case,
    PrimaryKey,
    Wildcard,
}

pub trait FromString {
    fn from_string(kw: String) -> Option<Self>
    where
        Self: Sized;
}

impl FromString for Keyword {
    fn from_string(input: String) -> Option<Self> {
        match input.as_str() {
            "select" => Some(Keyword::Select),
            "create" => Some(Keyword::Create),
            "update" => Some(Keyword::Update),
            "insert" => Some(Keyword::Insert),
            "delete" => Some(Keyword::Delete),
            "drop" => Some(Keyword::Drop),
            "from" => Some(Keyword::From),
            "into" => Some(Keyword::Into),
            "where" => Some(Keyword::Where),
            "distinct" => Some(Keyword::Distinct),
            "alter" => Some(Keyword::Alter),
            "add" => Some(Keyword::Add),
            "set" => Some(Keyword::Set),
            "truncate" => Some(Keyword::Truncate),
            "as" => Some(Keyword::As),
            "asc" => Some(Keyword::Asc),
            "desc" => Some(Keyword::Desc),
            "between" => Some(Keyword::Between),
            "having" => Some(Keyword::Having),
            "in" => Some(Keyword::In),
            "join" => Some(Keyword::Join),
            "exists" => Some(Keyword::Exists),
            "like" => Some(Keyword::Like),
            "case" => Some(Keyword::Case),
            "primary key" => Some(Keyword::PrimaryKey),
            "*" => Some(Keyword::Wildcard),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Punctuation {
    Comma,
    SemiColon,
}

impl FromString for Punctuation {
    fn from_string(input: String) -> Option<Self> {
        match input.as_str() {
            "," => Some(Punctuation::Comma),
            ";" => Some(Punctuation::SemiColon),
            _ => None,
        }
    }
}
