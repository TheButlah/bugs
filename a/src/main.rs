fn main() {
    // this only exists in tokio-rustls 0.26
    // This code fails to compile, because tls-listener is using tokio-rustls 0.25 instead.
    let _ = tls_listener::rustls::rustls::crypto::CryptoProvider::get_default;
}
