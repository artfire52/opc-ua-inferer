use std::fmt;


#[derive(Debug,Clone)]
pub enum MapperErrorKind{
    ParsingError,
    UnexpectedValue,
    MissingKey,
    KeyDerivation,
    RecvError,
    SendError,
    VariantError,
    RawRsaError,
}

#[derive(Clone)]
pub  struct MapperError{
    error_kind:MapperErrorKind,
    display:String,
}

impl MapperError{
    pub (crate) fn new(error_kind:MapperErrorKind,message:&str)-> MapperError{
        MapperError { error_kind, display: String::from(message) }
    }

    pub (crate) fn to_abstract(&self)->String{
        return format!("{:?}",self.error_kind)
    }

}
impl fmt::Debug for MapperError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f,"{:?} info:{}",self.error_kind,self.display)
}
}

impl fmt::Display for MapperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

