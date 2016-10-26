use super::Bytecode;

pub type Error = ();

pub fn check_bytecode(text: &Vec<String>, code: &Vec<u8>) -> Result<(), Error> {

    Ok(())
}

impl Bytecode {

    pub fn new(text: Vec<String>, code: Vec<u8>) -> Result<Bytecode, Error> {
        try!(check_bytecode(&text, &code));
        Ok(Bytecode {
            text: text,
            code: code,
        })
    }
}
