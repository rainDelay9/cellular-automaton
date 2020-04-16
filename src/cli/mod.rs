use crate::automaton::Automaton;
use exitfailure::ExitFailure;
use failure::ResultExt;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

pub fn cli() -> Result<Automaton, ExitFailure> {
    let opt = Opt::from_args();

    // get password
    let password: String = get_password(opt.password, opt.path_to_password)
        .with_context(|_| format!("could not determine password!"))?;

    // get aes scheme
    let aes_enum = aes_defs::openssl_index_to_enum(opt.scheme)
        .with_context(|_| format!("unsupported scheme!"))?;
    let scheme = aes::OpensslAesWrapper::new(&aes_enum);

    // initialize builder & encrypted-box
    let mut ebb = EncryptedBoxBuilder::new(scheme);
    let eb = ebb
        .set_password(password)
        .add_fields(&opt.fields[..])
        .build()?;

    // encrypt
    let enc = eb
        .encrypt()
        .with_context(|_| format!("encryption failed!"))?;
    println!("{}", base64::encode(&enc[..]));

    Ok(())
}

/// This tool allows you to encrypt any number of fields
/// with AES (choosing from a few flavors). It relies on
/// the openssl implementation. Output is in base 64. See
/// https://docs.rs/openssl/0.9.17/openssl/symm/struct.Cipher.html
/// for more information.
#[derive(StructOpt, Debug)]
#[structopt(name = "cellular automaton", version = "0.2.6", author = "")]
struct Opt {
    /// Path to cellular automata config file
    #[structopt(long = "config", parse(from_os_str), raw(required = "true"))]
    config: PathBuf,
    /// Fields to add
    #[structopt(long = "coordinates")]
    fields: Vec<String>,
}
