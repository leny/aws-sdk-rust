// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum SummaryChecksumAlgorithm {
    #[allow(missing_docs)] // documentation missing in model
    Summary,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for SummaryChecksumAlgorithm {
    fn from(s: &str) -> Self {
        match s {
            "SUMMARY" => SummaryChecksumAlgorithm::Summary,
            other => SummaryChecksumAlgorithm::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for SummaryChecksumAlgorithm {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(SummaryChecksumAlgorithm::from(s))
    }
}
impl SummaryChecksumAlgorithm {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            SummaryChecksumAlgorithm::Summary => "SUMMARY",
            SummaryChecksumAlgorithm::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["SUMMARY"]
    }
}
impl AsRef<str> for SummaryChecksumAlgorithm {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum DataChecksumAlgorithm {
    #[allow(missing_docs)] // documentation missing in model
    Sha256,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for DataChecksumAlgorithm {
    fn from(s: &str) -> Self {
        match s {
            "SHA256" => DataChecksumAlgorithm::Sha256,
            other => DataChecksumAlgorithm::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for DataChecksumAlgorithm {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(DataChecksumAlgorithm::from(s))
    }
}
impl DataChecksumAlgorithm {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            DataChecksumAlgorithm::Sha256 => "SHA256",
            DataChecksumAlgorithm::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["SHA256"]
    }
}
impl AsRef<str> for DataChecksumAlgorithm {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Object
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct BackupObject {
    /// Object name
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// Number of chunks in object
    #[doc(hidden)]
    pub chunks_count: std::option::Option<i64>,
    /// Metadata string associated with the Object
    #[doc(hidden)]
    pub metadata_string: std::option::Option<std::string::String>,
    /// Object checksum
    #[doc(hidden)]
    pub object_checksum: std::option::Option<std::string::String>,
    /// Checksum algorithm
    #[doc(hidden)]
    pub object_checksum_algorithm: std::option::Option<crate::model::SummaryChecksumAlgorithm>,
    /// Object token
    #[doc(hidden)]
    pub object_token: std::option::Option<std::string::String>,
}
impl BackupObject {
    /// Object name
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
    /// Number of chunks in object
    pub fn chunks_count(&self) -> std::option::Option<i64> {
        self.chunks_count
    }
    /// Metadata string associated with the Object
    pub fn metadata_string(&self) -> std::option::Option<&str> {
        self.metadata_string.as_deref()
    }
    /// Object checksum
    pub fn object_checksum(&self) -> std::option::Option<&str> {
        self.object_checksum.as_deref()
    }
    /// Checksum algorithm
    pub fn object_checksum_algorithm(
        &self,
    ) -> std::option::Option<&crate::model::SummaryChecksumAlgorithm> {
        self.object_checksum_algorithm.as_ref()
    }
    /// Object token
    pub fn object_token(&self) -> std::option::Option<&str> {
        self.object_token.as_deref()
    }
}
impl std::fmt::Debug for BackupObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("BackupObject");
        formatter.field("name", &self.name);
        formatter.field("chunks_count", &self.chunks_count);
        formatter.field("metadata_string", &self.metadata_string);
        formatter.field("object_checksum", &self.object_checksum);
        formatter.field("object_checksum_algorithm", &self.object_checksum_algorithm);
        formatter.field("object_token", &self.object_token);
        formatter.finish()
    }
}
/// See [`BackupObject`](crate::model::BackupObject).
pub mod backup_object {

    /// A builder for [`BackupObject`](crate::model::BackupObject).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) chunks_count: std::option::Option<i64>,
        pub(crate) metadata_string: std::option::Option<std::string::String>,
        pub(crate) object_checksum: std::option::Option<std::string::String>,
        pub(crate) object_checksum_algorithm:
            std::option::Option<crate::model::SummaryChecksumAlgorithm>,
        pub(crate) object_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Object name
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        /// Object name
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input;
            self
        }
        /// Number of chunks in object
        pub fn chunks_count(mut self, input: i64) -> Self {
            self.chunks_count = Some(input);
            self
        }
        /// Number of chunks in object
        pub fn set_chunks_count(mut self, input: std::option::Option<i64>) -> Self {
            self.chunks_count = input;
            self
        }
        /// Metadata string associated with the Object
        pub fn metadata_string(mut self, input: impl Into<std::string::String>) -> Self {
            self.metadata_string = Some(input.into());
            self
        }
        /// Metadata string associated with the Object
        pub fn set_metadata_string(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.metadata_string = input;
            self
        }
        /// Object checksum
        pub fn object_checksum(mut self, input: impl Into<std::string::String>) -> Self {
            self.object_checksum = Some(input.into());
            self
        }
        /// Object checksum
        pub fn set_object_checksum(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.object_checksum = input;
            self
        }
        /// Checksum algorithm
        pub fn object_checksum_algorithm(
            mut self,
            input: crate::model::SummaryChecksumAlgorithm,
        ) -> Self {
            self.object_checksum_algorithm = Some(input);
            self
        }
        /// Checksum algorithm
        pub fn set_object_checksum_algorithm(
            mut self,
            input: std::option::Option<crate::model::SummaryChecksumAlgorithm>,
        ) -> Self {
            self.object_checksum_algorithm = input;
            self
        }
        /// Object token
        pub fn object_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.object_token = Some(input.into());
            self
        }
        /// Object token
        pub fn set_object_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.object_token = input;
            self
        }
        /// Consumes the builder and constructs a [`BackupObject`](crate::model::BackupObject).
        pub fn build(self) -> crate::model::BackupObject {
            crate::model::BackupObject {
                name: self.name,
                chunks_count: self.chunks_count,
                metadata_string: self.metadata_string,
                object_checksum: self.object_checksum,
                object_checksum_algorithm: self.object_checksum_algorithm,
                object_token: self.object_token,
            }
        }
    }
}
impl BackupObject {
    /// Creates a new builder-style object to manufacture [`BackupObject`](crate::model::BackupObject).
    pub fn builder() -> crate::model::backup_object::Builder {
        crate::model::backup_object::Builder::default()
    }
}

/// Chunk
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Chunk {
    /// Chunk index
    #[doc(hidden)]
    pub index: i64,
    /// Chunk length
    #[doc(hidden)]
    pub length: i64,
    /// Chunk checksum
    #[doc(hidden)]
    pub checksum: std::option::Option<std::string::String>,
    /// Checksum algorithm
    #[doc(hidden)]
    pub checksum_algorithm: std::option::Option<crate::model::DataChecksumAlgorithm>,
    /// Chunk token
    #[doc(hidden)]
    pub chunk_token: std::option::Option<std::string::String>,
}
impl Chunk {
    /// Chunk index
    pub fn index(&self) -> i64 {
        self.index
    }
    /// Chunk length
    pub fn length(&self) -> i64 {
        self.length
    }
    /// Chunk checksum
    pub fn checksum(&self) -> std::option::Option<&str> {
        self.checksum.as_deref()
    }
    /// Checksum algorithm
    pub fn checksum_algorithm(&self) -> std::option::Option<&crate::model::DataChecksumAlgorithm> {
        self.checksum_algorithm.as_ref()
    }
    /// Chunk token
    pub fn chunk_token(&self) -> std::option::Option<&str> {
        self.chunk_token.as_deref()
    }
}
impl std::fmt::Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Chunk");
        formatter.field("index", &self.index);
        formatter.field("length", &self.length);
        formatter.field("checksum", &self.checksum);
        formatter.field("checksum_algorithm", &self.checksum_algorithm);
        formatter.field("chunk_token", &self.chunk_token);
        formatter.finish()
    }
}
/// See [`Chunk`](crate::model::Chunk).
pub mod chunk {

    /// A builder for [`Chunk`](crate::model::Chunk).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) index: std::option::Option<i64>,
        pub(crate) length: std::option::Option<i64>,
        pub(crate) checksum: std::option::Option<std::string::String>,
        pub(crate) checksum_algorithm: std::option::Option<crate::model::DataChecksumAlgorithm>,
        pub(crate) chunk_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Chunk index
        pub fn index(mut self, input: i64) -> Self {
            self.index = Some(input);
            self
        }
        /// Chunk index
        pub fn set_index(mut self, input: std::option::Option<i64>) -> Self {
            self.index = input;
            self
        }
        /// Chunk length
        pub fn length(mut self, input: i64) -> Self {
            self.length = Some(input);
            self
        }
        /// Chunk length
        pub fn set_length(mut self, input: std::option::Option<i64>) -> Self {
            self.length = input;
            self
        }
        /// Chunk checksum
        pub fn checksum(mut self, input: impl Into<std::string::String>) -> Self {
            self.checksum = Some(input.into());
            self
        }
        /// Chunk checksum
        pub fn set_checksum(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.checksum = input;
            self
        }
        /// Checksum algorithm
        pub fn checksum_algorithm(mut self, input: crate::model::DataChecksumAlgorithm) -> Self {
            self.checksum_algorithm = Some(input);
            self
        }
        /// Checksum algorithm
        pub fn set_checksum_algorithm(
            mut self,
            input: std::option::Option<crate::model::DataChecksumAlgorithm>,
        ) -> Self {
            self.checksum_algorithm = input;
            self
        }
        /// Chunk token
        pub fn chunk_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.chunk_token = Some(input.into());
            self
        }
        /// Chunk token
        pub fn set_chunk_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.chunk_token = input;
            self
        }
        /// Consumes the builder and constructs a [`Chunk`](crate::model::Chunk).
        pub fn build(self) -> crate::model::Chunk {
            crate::model::Chunk {
                index: self.index.unwrap_or_default(),
                length: self.length.unwrap_or_default(),
                checksum: self.checksum,
                checksum_algorithm: self.checksum_algorithm,
                chunk_token: self.chunk_token,
            }
        }
    }
}
impl Chunk {
    /// Creates a new builder-style object to manufacture [`Chunk`](crate::model::Chunk).
    pub fn builder() -> crate::model::chunk::Builder {
        crate::model::chunk::Builder::default()
    }
}
