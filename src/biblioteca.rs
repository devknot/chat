mod mensagem;

use mensagem::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mensagem() {
        let a = serialize(Mensagem::Texto);

        assert_eq!(Mensagem::Texto, deserialize(&a));
    }
}
