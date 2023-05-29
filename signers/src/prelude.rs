pub trait Signer {
    type Signature;

    async fn sign_message<S>(&self, message: S) -> Self::Signature
    where
        S: AsRef<[u8]>;
}
