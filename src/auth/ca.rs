use std::fs::{File, Permissions};

use openssl::asn1::Asn1Time;
use openssl::bn::{BigNum, MsbOption};
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;
use openssl::x509::{X509NameBuilder, X509};

pub struct CA {
    key: Vec<u8>,
    cert: Vec<u8>,
}

pub fn generate_ca() -> Result<(), Box<dyn std::error::Error>> {
    let rsa = Rsa::generate_with_e(4096, &BigNum::from_u32(65537u32).unwrap())?;
    let pkey = PKey::from_rsa(rsa)?;

    let mut name_builder = X509NameBuilder::new()?;
    name_builder.append_entry_by_text("C", "US")?;
    name_builder.append_entry_by_text("ST", "California")?;
    name_builder.append_entry_by_text("L", "Los Angeles")?;
    name_builder.append_entry_by_text("O", "Stevedore")?;
    name_builder.append_entry_by_text("CN", "Stevedore")?;
    let name = name_builder.build();

    let mut builder = X509::builder()?;
    builder.set_version(2)?;
    builder.set_subject_name(&name)?;
    builder.set_issuer_name(&name)?;
    builder.set_pubkey(&pkey)?;

    let serial_number = {
        let mut bn = BigNum::new()?;
        bn.rand(159, MsbOption::MAYBE_ZERO, false)?;
        bn.to_asn1_integer()?
    };
    builder.set_serial_number(&serial_number)?;

    let not_before = Asn1Time::days_from_now(0)?;
    let not_after = Asn1Time::days_from_now(365)?;
    builder.set_not_before(&not_before)?;
    builder.set_not_after(&not_after)?;

    builder.sign(&pkey, MessageDigest::sha256())?;

    let certificate = builder.build();

    let private_key = pkey.private_key_to_pem_pkcs8()?;
    std::fs::write("ca_key.pem", &private_key)?;

    let certificate = certificate.to_pem()?;
    std::fs::write("ca_cert.prm", &certificate)?;

    Ok(())
}