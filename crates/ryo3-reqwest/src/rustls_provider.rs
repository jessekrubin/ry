// err if no crypto provider is selected or multiple are selected
#[cfg(not(any(feature = "aws-lc-rs", feature = "ring")))]
compile_error!("need either `ring` or `aws-lc-rs` feature enabled for crypto provider");

// pub(crate) fn rustls_provider_install_default() {
//     #[cfg(feature = "aws-lc-rs")]
//     {
//         rustls::crypto::CryptoProvider::install_default(
//             rustls::crypto::aws_lc_rs::default_provider(),
//         )
//         .expect("Failed to install default crypto provider (aws-lc-rs)");
//     }

//     #[cfg(all(not(feature = "aws-lc-rs"), feature = "ring"))]
//     {
//         rustls::crypto::CryptoProvider::install_default(rustls::crypto::ring::default_provider())
//             .expect("Failed to install default crypto provider (ring)");
//     }
// }

// EXPERIMENTAL: fiddling with cipher suite preference

use rustls::crypto::CryptoProvider;
use rustls::{CipherSuite, SupportedCipherSuite};

/// aes-128-gcm before aes-256-gcm (same as chrome/boringssl) ~ rustls lists
/// aes-256 first and most servers go w/ the client's order; aes-256 is ~40%
/// more cpu per byte for us AND the server
fn prefer_aes_128(mut provider: CryptoProvider) -> CryptoProvider {
    provider.cipher_suites.sort_by_key(|cs| {
        let rank = match cs.suite() {
            CipherSuite::TLS13_AES_128_GCM_SHA256
            | CipherSuite::TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256
            | CipherSuite::TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256 => 0,
            CipherSuite::TLS13_AES_256_GCM_SHA384
            | CipherSuite::TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384
            | CipherSuite::TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384 => 1,
            _ => 2,
        };
        // stable sort ~ tls1.3 stays ahead of tls1.2
        (matches!(cs, SupportedCipherSuite::Tls12(_)), rank)
    });
    provider
}

pub(crate) fn rustls_provider_install_default() {
    #[cfg(feature = "aws-lc-rs")]
    {
        CryptoProvider::install_default(prefer_aes_128(
            rustls::crypto::aws_lc_rs::default_provider(),
        ))
        .expect("Failed to install default crypto provider (aws-lc-rs)");
    }

    #[cfg(all(not(feature = "aws-lc-rs"), feature = "ring"))]
    {
        CryptoProvider::install_default(prefer_aes_128(rustls::crypto::ring::default_provider()))
            .expect("Failed to install default crypto provider (ring)");
    }
}
