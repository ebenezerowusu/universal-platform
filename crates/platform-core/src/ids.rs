use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use uuid::Uuid;

macro_rules! uuid_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
        pub struct $name(Uuid);

        impl $name {
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }

            pub fn from_uuid(value: Uuid) -> Self {
                Self(value)
            }

            pub fn as_uuid(&self) -> &Uuid {
                &self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Display for $name {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
                write!(formatter, "{}", self.0)
            }
        }
    };
}

uuid_newtype!(OrganizationId);
uuid_newtype!(OrganizationMembershipId);
uuid_newtype!(UserId);
uuid_newtype!(TraceId);
