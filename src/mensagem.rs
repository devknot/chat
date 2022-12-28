#[derive(Debug, PartialEq)]
pub enum Mensagem {
    Texto,
    Sair,
}

pub fn serialize(m: Mensagem) -> Vec<u8> {
    match m {
        Mensagem::Texto => vec![0],
        Mensagem::Sair => vec![1],
    }
}

pub fn deserialize(b: &[u8]) -> Mensagem {
    match b[0] {
        0 => Mensagem::Texto,
        1 => Mensagem::Sair,
        _ => panic!("deu pau ao deserizar"),
    }
}

/*
enum Alerta {
    Desbinarizar,
}

pub trait Comunicação {
    fn binarizar(self) -> Vec<u8>;
    fn desbinarizar(&[u8]) -> Result<Self, Alerta>;
}
*/
