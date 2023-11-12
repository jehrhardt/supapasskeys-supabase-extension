use std::sync::Arc;

use uuid::Uuid;
use webauthn_rs::{
    prelude::{
        CreationChallengeResponse, Passkey, PasskeyRegistration, RegisterPublicKeyCredential, Url,
        WebauthnError,
    },
    Webauthn, WebauthnBuilder,
};

pub(crate) struct RelyingParty {
    pub(crate) name: String,
    pub(crate) origin: String,
}

pub(crate) struct User {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) display_name: String,
}

pub(crate) fn start_registration(
    relying_party: RelyingParty,
    user: User,
) -> (CreationChallengeResponse, Arc<PasskeyRegistration>) {
    match init_webauthn(&relying_party) {
        Ok(webauthn) => {
            match webauthn.start_passkey_registration(user.id, &user.name, &user.display_name, None)
            {
                Ok((ccr, skr)) => (ccr, Arc::new(skr)),
                Err(e) => panic!("Error: {}", e),
            }
        }
        Err(e) => panic!("Error: {}", e),
    }
}

pub(crate) fn finish_registration(
    relying_party: RelyingParty,
    reg: RegisterPublicKeyCredential,
    state: PasskeyRegistration,
) -> Passkey {
    match init_webauthn(&relying_party) {
        Ok(webauthn) => match webauthn.finish_passkey_registration(&reg, &state) {
            Ok(passkey) => passkey,
            Err(e) => panic!("Error: {}", e),
        },
        Err(e) => panic!("Error: {}", e),
    }
}

fn init_webauthn(relying_party: &RelyingParty) -> Result<Webauthn, WebauthnError> {
    let rp_origin = Url::parse(&relying_party.origin).unwrap();
    let rp_id = rp_origin.domain().unwrap();
    match WebauthnBuilder::new(rp_id, &rp_origin) {
        Ok(builder) => builder.rp_name(&relying_party.name).build(),
        Err(e) => panic!("Error: {}", e),
    }
}
