//! Protocol session
//! Defines role pre-session and session and their respective parameters

use curve25519_dalek::scalar::Scalar;
use monero::util::key::PrivateKey;
use secp256k1::key::SecretKey;

use crate::roles::{Alice, Bob, Role};

pub struct PreSession<R>
where
    R: Role + PreSessionParameters,
{
    pub parameters: <R as PreSessionParameters>::Parameters,
}

impl<R> PreSession<R>
where
    R: Role + PreSessionParameters,
{
    pub fn new(parameters: <R as PreSessionParameters>::Parameters) -> Self {
        PreSession { parameters }
    }

    pub fn get_params(&self) -> &<R as PreSessionParameters>::Parameters {
        &self.parameters
    }
}

impl<R> PreSession<R>
where
    R: Role + PreSessionParameters + SessionParameters,
{
    pub fn into_session(&self, session_params: <R as SessionParameters>::Parameters) -> Session<R> {
        Session::<R>::new(self.get_params().clone(), session_params)
    }
}

pub trait PreSessionParameters {
    type Parameters: Clone;
}

#[derive(Clone)]
pub struct AlicePreSessionParameters {
    pub destination_address: String,
}

impl PreSessionParameters for Alice {
    type Parameters = AlicePreSessionParameters;
}

#[derive(Clone)]
pub struct BobPreSessionParameters {
    pub refund_address: String,
}

impl PreSessionParameters for Bob {
    type Parameters = BobPreSessionParameters;
}

pub struct Session<R>
where
    R: Role + PreSessionParameters + SessionParameters,
{
    pub uuid: String,
    pub pre_session_parameters: <R as PreSessionParameters>::Parameters,
    pub parameters: <R as SessionParameters>::Parameters,
}

impl<R> Session<R>
where
    R: Role + PreSessionParameters + SessionParameters,
{
    /// Generate a new session for a swap role
    pub fn new(
        pre_session_parameters: <R as PreSessionParameters>::Parameters,
        parameters: <R as SessionParameters>::Parameters,
    ) -> Self {
        Session {
            uuid: String::from("session id"),
            pre_session_parameters,
            parameters,
        }
    }

    pub fn pre_session_params(&self) -> &<R as PreSessionParameters>::Parameters {
        &self.pre_session_parameters
    }

    pub fn get_params(&self) -> &<R as SessionParameters>::Parameters {
        &self.parameters
    }
}

pub trait SessionParameters {
    type Parameters;
}

pub struct AliceSessionParameters {
    pub buy: SecretKey,
    pub cancel: SecretKey,
    pub refund: SecretKey,
    pub punish: SecretKey,
    pub spend: PrivateKey,
    pub view: PrivateKey,
}

impl AliceSessionParameters {
    pub fn new() -> Self {
        let (buy, cancel, refund, punish) = {
            use secp256k1::rand::rngs::OsRng;
            let mut rng = OsRng::new().expect("OsRng");

            (
                SecretKey::new(&mut rng),
                SecretKey::new(&mut rng),
                SecretKey::new(&mut rng),
                SecretKey::new(&mut rng),
            )
        };

        let (spend, view) = {
            let mut csprng = rand_core::OsRng;

            (
                PrivateKey::from_scalar(Scalar::random(&mut csprng)),
                PrivateKey::from_scalar(Scalar::random(&mut csprng)),
            )
        };

        AliceSessionParameters {
            buy,
            cancel,
            refund,
            punish,
            spend,
            view,
        }
    }
}

impl SessionParameters for Alice {
    type Parameters = AliceSessionParameters;
}

pub struct BobSessionParameters {
    pub fund: SecretKey,
    pub buy: SecretKey,
    pub cancel: SecretKey,
    pub refund: SecretKey,
    pub spend: PrivateKey,
    pub view: PrivateKey,
}

impl BobSessionParameters {
    pub fn new() -> Self {
        let (fund, buy, cancel, refund) = {
            use secp256k1::rand::rngs::OsRng;
            let mut rng = OsRng::new().expect("OsRng");

            (
                SecretKey::new(&mut rng),
                SecretKey::new(&mut rng),
                SecretKey::new(&mut rng),
                SecretKey::new(&mut rng),
            )
        };

        let (spend, view) = {
            let mut csprng = rand_core::OsRng;

            (
                PrivateKey::from_scalar(Scalar::random(&mut csprng)),
                PrivateKey::from_scalar(Scalar::random(&mut csprng)),
            )
        };

        BobSessionParameters {
            fund,
            buy,
            cancel,
            refund,
            spend,
            view,
        }
    }
}

impl SessionParameters for Bob {
    type Parameters = BobSessionParameters;
}

#[cfg(test)]
mod tests {
    use super::{Alice, AlicePreSessionParameters, AliceSessionParameters, PreSession};

    #[test]
    fn create_presession() {
        let params = AlicePreSessionParameters {
            destination_address: String::from("bc1qndk902ka3266wzta9cnl4fgfcmhy7xqrdh26ka"),
        };
        let pre_session = PreSession::<Alice>::new(params);
        let session_params = AliceSessionParameters::new();
        let _session = pre_session.into_session(session_params);
    }
}
