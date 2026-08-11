use crate::traits::{SubsonicResponseTrait, SubsonicDataTrait};
use serde::{Serialize, Deserialize, de::{self, Visitor}};
use super::ErrorData;

/// Subsonic representation of a full 
/// [`subsonic-response`](https://opensubsonic.netlify.app/docs/responses/subsonic-response/) 
/// with nested data that implements [`Deserialize`]
///
/// Used in combination with the other Response Types for Subsonic to Deserialize Subsonic's JSON
/// responses
/// # Example
/// The following response with a nested [`license`](crate::models::License) element:
/// ```json
/// {
///     "subsonic-response": {
///         "status": "ok",
///         "version": "1.16.1",
///         "license": {
///             "valid": true,
///             "email": "demo@demo.org",
///             "licenseExpires": "2017-04-11T10:42:50.842Z",
///             "trialExpires": "2017-04-11T10:42:50.842Z"
///         }
///     }
/// }
/// ```
/// results in a case of `SubsonicResponse` with a nested [`License`](crate::models::License) struct
/// ```rust
/// # use subsonic::subsonic::models::{SubsonicResponse, SubsonicData};
/// # use subsonic::models::License;
/// #
/// # let data: SubsonicResponse<License> = serde_json::from_str(r#"
/// #     {
/// #         "subsonic-response": {
/// #             "status": "ok",
/// #             "version": "1.16.1",
/// #             "license": {
/// #                 "valid": true,
/// #                 "email": "demo@demo.org",
/// #                 "licenseExpires": "2017-04-11T10:42:50.842Z",
/// #                 "trialExpires": "2017-04-11T10:42:50.842Z"
/// #             }
/// #         }
/// #     }
/// # "#).unwrap();
/// # let tester = 
/// SubsonicResponse::<License> {
///     subsonic_response: SubsonicData::<License> {
///         status: "ok".into(),
///         version: "1.16.1".into(),
///         additional: Ok(License {
///             valid: true,
///             email: "demo@demo.org".into(),
///             license_expires: "2017-04-11T10:42:50.842Z".into(),
///             trial_expires: "2017-04-11T10:42:50.842Z".into()
///         })
///     }
/// }
/// # ;
/// # assert_eq!(data, tester);
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct SubsonicResponse<T> 
{
    pub subsonic_response: SubsonicData<T>,
}

#[allow(refining_impl_trait)]
impl<T> SubsonicResponseTrait<T, ErrorData> for SubsonicResponse<T> {
    fn subsonic_response(&self) -> &SubsonicData<T> {
        &self.subsonic_response
    }

    fn into_subsonic_data(self) -> SubsonicData<T> {
       self.subsonic_response 
    }
}

/// Wrapper around a Struct that implements [`Deserialize`] for Subsonic
///
/// Used in combination with the other Response Types for Subsonic to Deserialize Subsonic's JSON
/// responses
/// # Example
/// The following response with a nested [`license`](crate::models::License) element:
/// ```json
/// {
///     "status": "ok",
///     "version": "1.16.1",
///     "license": {
///         "valid": true,
///         "email": "demo@demo.org",
///         "licenseExpires": "2017-04-11T10:42:50.842Z",
///         "trialExpires": "2017-04-11T10:42:50.842Z"
///     }
/// }
/// ```
/// results in a case of `SubsonicData` with a nested [`License`](crate::models::License) struct
/// ```rust
/// # use subsonic::subsonic::models::SubsonicData;
/// # use subsonic::models::License;
/// #
/// # let data: SubsonicData<License> = serde_json::from_str(r#"
/// #     {
/// #         "status": "ok",
/// #         "version": "1.16.1",
/// #         "license": {
/// #             "valid": true,
/// #             "email": "demo@demo.org",
/// #             "licenseExpires": "2017-04-11T10:42:50.842Z",
/// #             "trialExpires": "2017-04-11T10:42:50.842Z"
/// #         }
/// #     }
/// # "#).unwrap();
/// # let tester = 
/// SubsonicData::<License> {
///     status: "ok".into(),
///     version: "1.16.1".into(),
///     additional: Ok(License {
///         valid: true,
///         email: "demo@demo.org".into(),
///         license_expires: "2017-04-11T10:42:50.842Z".into(),
///         trial_expires: "2017-04-11T10:42:50.842Z".into()
///     })
/// }
/// # ;
/// # assert_eq!(data, tester);
/// ```
/// # Error Example
/// The following response which contains an error:
/// ```json
/// {
///     "status": "ok",
///     "version": "1.16.1",
///     "error": {
///         "code": 42,
///         "message": "Authentication mechanism not supported. Use API keys",
///     }
/// }
/// ```
/// results in a case of `OpenSubsonicData` with a nested [`ErrorData`](crate::subsonic::models::ErrorData) struct
/// ```rust
/// # use subsonic::subsonic::models::SubsonicData;
/// # use subsonic::subsonic::models::ErrorData;
/// # use subsonic::models::SubsonicErrorCode;
/// # use subsonic::models::License;
/// #
/// # let data: SubsonicData<License> = serde_json::from_str(r#"
/// # {
/// #     "status": "ok",
/// #     "version": "1.16.1",
/// #     "error": {
/// #         "code": 42,
/// #         "message": "Authentication mechanism not supported. Use API keys"
/// #     }
/// # }
/// # "#).unwrap();
/// # let tester = 
/// SubsonicData::<License> {
///     status: "ok".into(),
///     version: "1.16.1".into(),
///     additional: Err(ErrorData {
///         code: SubsonicErrorCode::AuthMechanismNotSupported,
///         message: "Authentication mechanism not supported. Use API keys".into(),
///     })
/// }
/// # ;
/// # assert_eq!(data, tester);
/// ```
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct SubsonicData<T> 
{
    /// The command result. `ok` or `failed`
    pub status: Box<str>,
    /// The server supported Subsonic API version.
    pub version: Box<str>,
    /// The nested Data provided in case of a success
    pub additional: Result<T, ErrorData>,
}

impl<T> SubsonicDataTrait<T, ErrorData> for SubsonicData<T> {
    fn status(&self) -> &str {
        &self.status
    }
    fn version(&self) -> &str {
        &self.version
    }
    fn additional(&self) -> Result<&T, &ErrorData> {
        self.additional.as_ref()
    }
    fn into_additional(self) -> Result<T, ErrorData> {
        self.additional
    }
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
enum UntaggedResult<D> {
    Ok(D),
    Err(ErrorData),
}

impl<D> From<UntaggedResult<D>> for Result<D, ErrorData> {
    fn from(val: UntaggedResult<D>) -> Self {
        match val {
            UntaggedResult::Ok(v) => Ok(v),
            UntaggedResult::Err(v) => Err(v)
        }
    }
}

impl<'de, T> Deserialize<'de> for SubsonicData<T> 
where 
    T: de::Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        enum Field { Status, Version, Additional }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de> 
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`status`, `version`, or any subsonic compatible field")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                        where
                            E: serde::de::Error, 
                    {
                        match value {
                            "status" => Ok(Field::Status),
                            "version" => Ok(Field::Version),
                            _ => Ok(Field::Additional),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct FoobarVisitor<T> {
            phantom: std::marker::PhantomData<T>
        }

        impl<'de, T> Visitor<'de> for FoobarVisitor<T>
        where
            T: de::Deserialize<'de>
        {
            type Value = SubsonicData<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Foobar")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::MapAccess<'de>, 
            {
                let mut status = None;
                let mut version = None;
                let mut additional: Option<UntaggedResult<T>> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Status => {
                            if status.is_some() {
                                return Err(de::Error::duplicate_field("status"));
                            }
                            status = Some(map.next_value()?);
                        },
                        Field::Version => {
                            if version.is_some() {
                                return Err(de::Error::duplicate_field("version"));
                            }
                            version = Some(map.next_value()?);
                        },
                        Field::Additional => {
                            if additional.is_some() {
                                return Err(de::Error::duplicate_field("additional"));
                            }
                            additional = Some(map.next_value()?);
                        },
                    }
                }
                let status = status.ok_or_else(|| de::Error::missing_field("status"))?;
                let version = version.ok_or_else(|| de::Error::missing_field("version"))?;
                // let additional = additional.ok_or_else(|| de::Error::missing_field("additional"))?;
                let additional = additional
                    .or_else(|| serde_json::from_str("null").ok())
                    .ok_or_else(|| de::Error::missing_field("additional"))?;
                Ok(SubsonicData { status, version, additional: additional.into() })
            }
        }

        const FIELDS: &[&str] = &["status", "version", "*"];
        deserializer.deserialize_struct("SubsonicData", FIELDS, FoobarVisitor { phantom: std::marker::PhantomData })
    }
}

