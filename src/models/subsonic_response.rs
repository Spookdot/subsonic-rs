use serde::{Serialize, Deserialize, de::{self, Visitor}};

// TODO rename Trait
pub trait SubsonicResponseTrait<T> {
    fn subsonic_response(&self) -> &impl SubsonicDataTrait<T>;
    fn into_subsonic_data(self) -> impl SubsonicDataTrait<T>;
}

// TODO rename Trait
pub trait SubsonicDataTrait<T> {
    fn status(&self) -> &str;
    fn version(&self) -> &str;
    fn additional(&self) -> &T;
    fn into_additional(self) -> T;
}

/// Representation of a full [`subsonic-response`](https://opensubsonic.netlify.app/docs/responses/subsonic-response/) 
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
/// # use subsonic::models::{SubsonicResponse, SubsonicData, License};
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
///         additional: License {
///             valid: true,
///             email: "demo@demo.org".into(),
///             license_expires: "2017-04-11T10:42:50.842Z".into(),
///             trial_expires: "2017-04-11T10:42:50.842Z".into()
///         }
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
impl<T> SubsonicResponseTrait<T> for SubsonicResponse<T> {
    fn subsonic_response(&self) -> &SubsonicData<T> {
        &self.subsonic_response
    }

    fn into_subsonic_data(self) -> SubsonicData<T> {
       self.subsonic_response 
    }
}

/// Wrapper around a Struct that implements [`Deserialize`]
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
/// # use subsonic::models::{SubsonicData, License};
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
///     additional: License {
///         valid: true,
///         email: "demo@demo.org".into(),
///         license_expires: "2017-04-11T10:42:50.842Z".into(),
///         trial_expires: "2017-04-11T10:42:50.842Z".into()
///     }
/// }
/// # ;
/// # assert_eq!(data, tester);
/// ```
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct SubsonicData<T> 
{
    pub status: Box<str>,
    pub version: Box<str>,
    pub additional: T,
}

impl<T> SubsonicDataTrait<T> for SubsonicData<T> {
    fn status(&self) -> &str {
        &self.status
    }
    fn version(&self) -> &str {
        &self.version
    }
    fn additional(&self) -> &T {
        &self.additional
    }
    fn into_additional(self) -> T {
        self.additional
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
                let mut additional = None;
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
                let additional = additional.ok_or_else(|| de::Error::missing_field("additional"))?;
                Ok(SubsonicData { status, version, additional })
            }
        }

        const FIELDS: &[&str] = &["status", "version", "*"];
        deserializer.deserialize_struct("SubsonicData", FIELDS, FoobarVisitor { phantom: std::marker::PhantomData })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct OpenSubsonicResponse<T> 
{
    pub subsonic_response: OpenSubsonicData<T>,
}

#[allow(refining_impl_trait)]
impl<T> SubsonicResponseTrait<T> for OpenSubsonicResponse<T> {
    fn subsonic_response(&self) -> &OpenSubsonicData<T> {
        &self.subsonic_response
    }
    fn into_subsonic_data(self) -> OpenSubsonicData<T> {
        self.subsonic_response
    }
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct OpenSubsonicData<T> 
{
    pub status: Box<str>,
    pub version: Box<str>,
    pub open_subsonic: bool,
    pub server_version: Box<str>,
    pub type_: Box<str>,
    pub additional: T,
}

impl<T> SubsonicDataTrait<T> for OpenSubsonicData<T> {
    fn status(&self) -> &str {
        &self.status
    }
    fn version(&self) -> &str {
        &self.version
    }
    fn additional(&self) -> &T {
        &self.additional
    }
    fn into_additional(self) -> T {
        self.additional
    }
}

impl<'de, T> Deserialize<'de> for OpenSubsonicData<T> 
where 
    T: de::Deserialize<'de>
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
            T: de::Deserialize<'de>
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
                let mut additional = None;
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
                let additional = additional.ok_or_else(|| de::Error::missing_field("additional"))?;
                Ok(OpenSubsonicData { status, version, open_subsonic, server_version, type_, additional })
            }
        }

        const FIELDS: &[&str] = &["status", "version", "openSubsonic", "serverVersion", "type", "*"];
        deserializer.deserialize_struct("OpenSubsonicData", FIELDS, FoobarVisitor { phantom: std::marker::PhantomData })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;

    #[derive(Deserialize)]
    struct Test {
        text: Box<str>
    }

    #[test]
    fn test() {
        let data: SubsonicData<Test> = serde_json::from_str(r#"
            {
                "status": "ok",
                "version": "1.16.1",
                "test": {
                    "text": "Hello World"
                }
            }
        "#).unwrap();
        assert_eq!(data.additional.text, "Hello World".into());
    }

    // #[test]
    // fn ping() {
    //     let data: SubsonicData<()> = serde_json::from_str(r#"
    //         {
    //             "status": "ok",
    //             "version": "1.16.1"
    //         }
    //     "#).unwrap();
    //     assert_eq!(data.version, "1.16.1".into());
    // }

    #[test]
    fn search3() {
        let data: OpenSubsonicData<SearchResult3> = serde_json::from_str(r#"
            {
    "status": "ok",
    "version": "1.16.1",
    "type": "AwesomeServerName",
    "serverVersion": "0.1.3 (tag)",
    "openSubsonic": true,
    "searchResult3": {
      "artist": [
        {
          "id": "37ec820ca7193e17040c98f7da7c4b51",
          "name": "2 Mello",
          "coverArt": "ar-37ec820ca7193e17040c98f7da7c4b51_0",
          "albumCount": 1,
          "userRating": 5,
          "artistImageUrl": "https://demo.org/image.jpg"
        }
      ],
      "album": [
        {
          "id": "ad0f112b6dcf83de5e9cae85d07f0d35",
          "name": "8-bit lagerfeuer",
          "artist": "pornophonique",
          "year": 2007,
          "coverArt": "al-ad0f112b6dcf83de5e9cae85d07f0d35_640a93a8",
          "starred": "2023-03-22T01:51:06Z",
          "duration": 1954,
          "playCount": 97,
          "played": "2023-03-28T00:45:13Z",
          "created": "2023-03-10T02:19:35.784818075Z",
          "artistId": "91c3901ac465b9efc439e4be4270c2b6",
          "userRating": 4,
          "songCount": 8
        }
      ],
      "song": [
        {
          "id": "082f435a363c32c57d5edb6a678a28d4",
          "parent": "e8a0685e3f3ec6f251649af2b58b8617",
          "isDir": false,
          "title": "\"polar expedition\"",
          "album": "Live at The Casbah - 2005-04-29",
          "artist": "The New Deal",
          "track": 4,
          "year": 2005,
          "coverArt": "mf-082f435a363c32c57d5edb6a678a28d4_6410b3ce",
          "size": 19866778,
          "contentType": "audio/flac",
          "suffix": "flac",
          "starred": "2023-03-27T09:45:27Z",
          "duration": 178,
          "bitRate": 880,
          "bitDepth": 16,
          "samplingRate": 44100,
          "channelCount": 2,
          "path": "The New Deal/Live at The Casbah - 2005-04-29/04 - \"polar expedition\".flac",
          "playCount": 8,
          "played": "2023-03-26T22:27:46Z",
          "discNumber": 1,
          "created": "2023-03-14T17:51:22.112827504Z",
          "albumId": "e8a0685e3f3ec6f251649af2b58b8617",
          "artistId": "97e0398acf63f9fb930d7d4ce209a52b",
          "type": "music",
          "isVideo": false
        }
      ]
    }
  }

        "#).unwrap();
        assert_eq!(data.additional.artist[0].id, "37ec820ca7193e17040c98f7da7c4b51");
    }
}
