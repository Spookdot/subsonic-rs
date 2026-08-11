use serde::{Serialize, Deserialize, de::{self, Visitor}};
use crate::{opensubsonic::models::ErrorData, traits::{SubsonicDataTrait, SubsonicResponseTrait}};

/// OpenSubsonic representation of a full 
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
///         "type": "AwesomeServerName",
///         "serverVersion": "0.1.3 (tag)",
///         "openSubsonic": true,
///         "license": {
///             "valid": true,
///             "email": "demo@demo.org",
///             "licenseExpires": "2017-04-11T10:42:50.842Z",
///             "trialExpires": "2017-04-11T10:42:50.842Z"
///         }
///     }
/// }
/// ```
/// results in a case of `OpenSubsonicResponse` with a nested [`License`](crate::models::License) struct
/// ```rust
/// # use subsonic::opensubsonic::models::{OpenSubsonicResponse, OpenSubsonicData};
/// # use subsonic::models::License;
/// #
/// # let data: OpenSubsonicResponse<License> = serde_json::from_str(r#"
/// #     {
/// #         "subsonic-response": {
/// #             "status": "ok",
/// #             "version": "1.16.1",
/// #             "type": "AwesomeServerName",
/// #             "serverVersion": "0.1.3 (tag)",
/// #             "openSubsonic": true,
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
/// OpenSubsonicResponse::<License> {
///     subsonic_response: OpenSubsonicData::<License> {
///         status: "ok".into(),
///         version: "1.16.1".into(),
///         type_: "AwesomeServerName".into(),
///         server_version: "0.1.3 (tag)".into(),
///         open_subsonic: true,
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
pub struct OpenSubsonicResponse<T: Serialize> 
{
    pub subsonic_response: OpenSubsonicData<T>,
}

#[allow(refining_impl_trait)]
impl<T: Serialize> SubsonicResponseTrait<T, ErrorData> for OpenSubsonicResponse<T> {
    fn subsonic_response(&self) -> &OpenSubsonicData<T> {
        &self.subsonic_response
    }
    fn into_subsonic_data(self) -> OpenSubsonicData<T> {
        self.subsonic_response
    }
}

/// Wrapper around a Struct that implements [`Deserialize`] for OpenSubsonic
///
/// Used in combination with the other Response Types for Subsonic to Deserialize Subsonic's JSON
/// responses
/// # Example
/// The following response with a nested [`license`](crate::models::License) element:
/// ```json
/// {
///     "status": "ok",
///     "version": "1.16.1",
///     "type": "AwesomeServerName",
///     "serverVersion": "0.1.3 (tag)",
///     "openSubsonic": true,
///     "license": {
///         "valid": true,
///         "email": "demo@demo.org",
///         "licenseExpires": "2017-04-11T10:42:50.842Z",
///         "trialExpires": "2017-04-11T10:42:50.842Z"
///     }
/// }
/// ```
/// results in a case of `OpenSubsonicData` with a nested [`License`](crate::models::License) struct
/// ```rust
/// # use subsonic::opensubsonic::models::OpenSubsonicData;
/// # use subsonic::models::License;
/// #
/// # let data: OpenSubsonicData<License> = serde_json::from_str(r#"
/// #     {
/// #         "status": "ok",
/// #         "version": "1.16.1",
/// #         "type": "AwesomeServerName",
/// #         "serverVersion": "0.1.3 (tag)",
/// #         "openSubsonic": true,
/// #         "license": {
/// #             "valid": true,
/// #             "email": "demo@demo.org",
/// #             "licenseExpires": "2017-04-11T10:42:50.842Z",
/// #             "trialExpires": "2017-04-11T10:42:50.842Z"
/// #         }
/// #     }
/// # "#).unwrap();
/// # let tester = 
/// OpenSubsonicData::<License> {
///     status: "ok".into(),
///     version: "1.16.1".into(),
///     type_: "AwesomeServerName".into(),
///     server_version: "0.1.3 (tag)".into(),
///     open_subsonic: true,
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
///     "type": "AwesomeServerName",
///     "serverVersion": "0.1.3 (tag)",
///     "openSubsonic": true,
///     "error": {
///         "code": 42,
///         "message": "Authentication mechanism not supported. Use API keys",
///         "helpUrl": "https://example.org/users/apiKey"
///     }
/// }
/// ```
/// results in a case of `OpenSubsonicData` with a nested [`ErrorData`](crate::opensubsonic::models::ErrorData) struct
/// ```rust
/// # use subsonic::opensubsonic::models::OpenSubsonicData;
/// # use subsonic::opensubsonic::models::ErrorData;
/// # use subsonic::models::SubsonicErrorCode;
/// # use subsonic::models::License;
/// #
/// # let data: OpenSubsonicData<License> = serde_json::from_str(r#"
/// # {
/// #     "status": "ok",
/// #     "version": "1.16.1",
/// #     "type": "AwesomeServerName",
/// #     "serverVersion": "0.1.3 (tag)",
/// #     "openSubsonic": true,
/// #     "error": {
/// #         "code": 42,
/// #         "message": "Authentication mechanism not supported. Use API keys",
/// #         "helpUrl": "https://example.org/users/apiKey"
/// #     }
/// # }
/// # "#).unwrap();
/// # let tester = 
/// OpenSubsonicData::<License> {
///     status: "ok".into(),
///     version: "1.16.1".into(),
///     type_: "AwesomeServerName".into(),
///     server_version: "0.1.3 (tag)".into(),
///     open_subsonic: true,
///     additional: Err(ErrorData {
///         code: SubsonicErrorCode::AuthMechanismNotSupported,
///         message: "Authentication mechanism not supported. Use API keys".into(),
///         help_url: Some("https://example.org/users/apiKey".into()),
///     })
/// }
/// # ;
/// # assert_eq!(data, tester);
/// ```
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct OpenSubsonicData<T: Serialize> 
{
    /// The command result. `ok` or `failed`
    pub status: Box<str>,
    /// The server supported Subsonic API version.
    pub version: Box<str>,
    /// Must return true if the server supports OpenSubsonic API v1
    pub open_subsonic: bool,
    /// The server actual version. \[Ex: `1.2.3 (beta)`\]
    pub server_version: Box<str>,
    /// The server actual name. \[Ex: `Navidrome` or `gonic`\]
    pub type_: Box<str>,
    /// The nested Data provided in case of a success
    pub additional: Result<T, ErrorData>,
}

impl<T: Serialize> SubsonicDataTrait<T, ErrorData> for OpenSubsonicData<T> {
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

impl<'de, T> Deserialize<'de> for OpenSubsonicData<T> 
where 
    T: de::Deserialize<'de> + Serialize
{

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        enum Field { Status, Version, OpenSubsonic, ServerVersion, Type, Additional }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de> 
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`status`, `version`, `openSubsonic`, `serverVersion`, `type` or any opensubsonic compatible field")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                        where
                            E: serde::de::Error, 
                    {
                        match value {
                            "status" => Ok(Field::Status),
                            "version" => Ok(Field::Version),
                            "openSubsonic" => Ok(Field::OpenSubsonic),
                            "serverVersion" => Ok(Field::ServerVersion),
                            "type" => Ok(Field::Type),
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
            T: de::Deserialize<'de> + Serialize
        {
            type Value = OpenSubsonicData<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Foobar")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::MapAccess<'de>, 
            {
                let mut status = None;
                let mut version = None;
                let mut open_subsonic = None;
                let mut server_version = None;
                let mut type_ = None;
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
                        Field::OpenSubsonic => {
                            if open_subsonic.is_some() {
                                return Err(de::Error::duplicate_field("open_subsonic"));
                            }
                            open_subsonic = Some(map.next_value()?);
                        },
                        Field::ServerVersion => {
                            if server_version.is_some() {
                                return Err(de::Error::duplicate_field("server_version"));
                            }
                            server_version = Some(map.next_value()?);
                        },
                        Field::Type => {
                            if type_.is_some() {
                                return Err(de::Error::duplicate_field("type_"));
                            }
                            type_ = Some(map.next_value()?);
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
                let open_subsonic = open_subsonic.ok_or_else(|| de::Error::missing_field("open_subsonic"))?;
                let server_version = server_version.ok_or_else(|| de::Error::missing_field("server_version"))?;
                let type_ = type_.ok_or_else(|| de::Error::missing_field("type_"))?;
                // let additional = additional.ok_or_else(|| de::Error::missing_field("additional"))?;
                let additional = additional.unwrap_or_else(|| serde_json::from_str("").unwrap());
                Ok(OpenSubsonicData { status, version, open_subsonic, server_version, type_, additional: additional.into() })
            }
        }

        const FIELDS: &[&str] = &["status", "version", "openSubsonic", "serverVersion", "type", "*"];
        deserializer.deserialize_struct("OpenSubsonicData", FIELDS, FoobarVisitor { phantom: std::marker::PhantomData })
    }
}
