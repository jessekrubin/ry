// err if no crypto provider is selected or multiple are selected
#[cfg(not(any(feature = "aws-lc-rs", feature = "ring")))]
compile_error!("need either `ring` or `aws-lc-rs` feature enabled for crypto provider");

pub(crate) fn rustls_provider_install_default() {
    #[cfg(feature = "aws-lc-rs")]
    {
        rustls::crypto::CryptoProvider::install_default(
            rustls::crypto::aws_lc_rs::default_provider(),
        )
        .expect("Failed to install default crypto provider (aws-lc-rs)");
    }

    #[cfg(all(not(feature = "aws-lc-rs"), feature = "ring"))]
    {
        rustls::crypto::CryptoProvider::install_default(rustls::crypto::ring::default_provider())
            .expect("Failed to install default crypto provider (ring)");
    }
}
