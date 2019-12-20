use std::{
    convert::TryFrom,
    path::{Component, PathBuf},
};
use thiserror::Error;
use uuid::Uuid;

// repr of an object id / {path, relative}.
// PathBuf format: `{h:1.b}/{h:2.b}/{rest (14 bytes)}`
struct ObjectId(pub Uuid);

impl From<Uuid> for ObjectId {
    fn from(x: Uuid) -> Self {
        ObjectId(x)
    }
}

impl Into<Uuid> for ObjectId {
    fn into(self) -> Uuid {
        self.0
    }
}

#[derive(Clone, Debug, Error)]
enum InvalidObjectId {
    #[error("non-UTF8 path")]
    NonUtf8,
    #[error("invalid UUID")]
    InvalidUuid(#[from] uuid::Error),
}

impl TryFrom<PathBuf> for ObjectId {
    type Error = InvalidObjectId;

    fn try_from(x: PathBuf) -> Result<Self, InvalidObjectId> {
        Ok(Self(Uuid::parse_str(
            &x.components()
                .try_fold::<_, _, Result<String, InvalidObjectId>>(
                    String::new(),
                    |mut acc, i| {
                        if let Component::Normal(x) = i {
                            acc += x.to_str().ok_or(InvalidObjectId::NonUtf8)?;
                        }
                        Ok(acc)
                    },
                )?,
        )?))
    }
}

impl Into<PathBuf> for ObjectId {
    fn into(self) -> PathBuf {
        let mut buf = Uuid::encode_buffer();
        let lowfmt = self.0.to_simple().encode_lower(&mut buf);
        let mut ret = PathBuf::from(&lowfmt[0..2]);
        ret.push(&lowfmt[2..4]);
        ret.push(&lowfmt[4..]);
        ret
    }
}
