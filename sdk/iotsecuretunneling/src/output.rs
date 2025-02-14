// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UntagResourceOutput {}
/// See [`UntagResourceOutput`](crate::output::UntagResourceOutput).
pub mod untag_resource_output {

    /// A builder for [`UntagResourceOutput`](crate::output::UntagResourceOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`UntagResourceOutput`](crate::output::UntagResourceOutput).
        pub fn build(self) -> crate::output::UntagResourceOutput {
            crate::output::UntagResourceOutput {}
        }
    }
}
impl UntagResourceOutput {
    /// Creates a new builder-style object to manufacture [`UntagResourceOutput`](crate::output::UntagResourceOutput).
    pub fn builder() -> crate::output::untag_resource_output::Builder {
        crate::output::untag_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct TagResourceOutput {}
/// See [`TagResourceOutput`](crate::output::TagResourceOutput).
pub mod tag_resource_output {

    /// A builder for [`TagResourceOutput`](crate::output::TagResourceOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`TagResourceOutput`](crate::output::TagResourceOutput).
        pub fn build(self) -> crate::output::TagResourceOutput {
            crate::output::TagResourceOutput {}
        }
    }
}
impl TagResourceOutput {
    /// Creates a new builder-style object to manufacture [`TagResourceOutput`](crate::output::TagResourceOutput).
    pub fn builder() -> crate::output::tag_resource_output::Builder {
        crate::output::tag_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct RotateTunnelAccessTokenOutput {
    /// <p>The Amazon Resource Name for the tunnel.</p>
    #[doc(hidden)]
    pub tunnel_arn: std::option::Option<std::string::String>,
    /// <p>The client access token that the source local proxy uses to connect to IoT Secure Tunneling.</p>
    #[doc(hidden)]
    pub source_access_token: std::option::Option<std::string::String>,
    /// <p>The client access token that the destination local proxy uses to connect to IoT Secure Tunneling.</p>
    #[doc(hidden)]
    pub destination_access_token: std::option::Option<std::string::String>,
}
impl RotateTunnelAccessTokenOutput {
    /// <p>The Amazon Resource Name for the tunnel.</p>
    pub fn tunnel_arn(&self) -> std::option::Option<&str> {
        self.tunnel_arn.as_deref()
    }
    /// <p>The client access token that the source local proxy uses to connect to IoT Secure Tunneling.</p>
    pub fn source_access_token(&self) -> std::option::Option<&str> {
        self.source_access_token.as_deref()
    }
    /// <p>The client access token that the destination local proxy uses to connect to IoT Secure Tunneling.</p>
    pub fn destination_access_token(&self) -> std::option::Option<&str> {
        self.destination_access_token.as_deref()
    }
}
impl std::fmt::Debug for RotateTunnelAccessTokenOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("RotateTunnelAccessTokenOutput");
        formatter.field("tunnel_arn", &self.tunnel_arn);
        formatter.field("source_access_token", &"*** Sensitive Data Redacted ***");
        formatter.field(
            "destination_access_token",
            &"*** Sensitive Data Redacted ***",
        );
        formatter.finish()
    }
}
/// See [`RotateTunnelAccessTokenOutput`](crate::output::RotateTunnelAccessTokenOutput).
pub mod rotate_tunnel_access_token_output {

    /// A builder for [`RotateTunnelAccessTokenOutput`](crate::output::RotateTunnelAccessTokenOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default)]
    pub struct Builder {
        pub(crate) tunnel_arn: std::option::Option<std::string::String>,
        pub(crate) source_access_token: std::option::Option<std::string::String>,
        pub(crate) destination_access_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The Amazon Resource Name for the tunnel.</p>
        pub fn tunnel_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.tunnel_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name for the tunnel.</p>
        pub fn set_tunnel_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.tunnel_arn = input;
            self
        }
        /// <p>The client access token that the source local proxy uses to connect to IoT Secure Tunneling.</p>
        pub fn source_access_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.source_access_token = Some(input.into());
            self
        }
        /// <p>The client access token that the source local proxy uses to connect to IoT Secure Tunneling.</p>
        pub fn set_source_access_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.source_access_token = input;
            self
        }
        /// <p>The client access token that the destination local proxy uses to connect to IoT Secure Tunneling.</p>
        pub fn destination_access_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.destination_access_token = Some(input.into());
            self
        }
        /// <p>The client access token that the destination local proxy uses to connect to IoT Secure Tunneling.</p>
        pub fn set_destination_access_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.destination_access_token = input;
            self
        }
        /// Consumes the builder and constructs a [`RotateTunnelAccessTokenOutput`](crate::output::RotateTunnelAccessTokenOutput).
        pub fn build(self) -> crate::output::RotateTunnelAccessTokenOutput {
            crate::output::RotateTunnelAccessTokenOutput {
                tunnel_arn: self.tunnel_arn,
                source_access_token: self.source_access_token,
                destination_access_token: self.destination_access_token,
            }
        }
    }
    impl std::fmt::Debug for Builder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut formatter = f.debug_struct("Builder");
            formatter.field("tunnel_arn", &self.tunnel_arn);
            formatter.field("source_access_token", &"*** Sensitive Data Redacted ***");
            formatter.field(
                "destination_access_token",
                &"*** Sensitive Data Redacted ***",
            );
            formatter.finish()
        }
    }
}
impl RotateTunnelAccessTokenOutput {
    /// Creates a new builder-style object to manufacture [`RotateTunnelAccessTokenOutput`](crate::output::RotateTunnelAccessTokenOutput).
    pub fn builder() -> crate::output::rotate_tunnel_access_token_output::Builder {
        crate::output::rotate_tunnel_access_token_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct OpenTunnelOutput {
    /// <p>A unique alpha-numeric tunnel ID.</p>
    #[doc(hidden)]
    pub tunnel_id: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name for the tunnel.</p>
    #[doc(hidden)]
    pub tunnel_arn: std::option::Option<std::string::String>,
    /// <p>The access token the source local proxy uses to connect to IoT Secure Tunneling.</p>
    #[doc(hidden)]
    pub source_access_token: std::option::Option<std::string::String>,
    /// <p>The access token the destination local proxy uses to connect to IoT Secure Tunneling.</p>
    #[doc(hidden)]
    pub destination_access_token: std::option::Option<std::string::String>,
}
impl OpenTunnelOutput {
    /// <p>A unique alpha-numeric tunnel ID.</p>
    pub fn tunnel_id(&self) -> std::option::Option<&str> {
        self.tunnel_id.as_deref()
    }
    /// <p>The Amazon Resource Name for the tunnel.</p>
    pub fn tunnel_arn(&self) -> std::option::Option<&str> {
        self.tunnel_arn.as_deref()
    }
    /// <p>The access token the source local proxy uses to connect to IoT Secure Tunneling.</p>
    pub fn source_access_token(&self) -> std::option::Option<&str> {
        self.source_access_token.as_deref()
    }
    /// <p>The access token the destination local proxy uses to connect to IoT Secure Tunneling.</p>
    pub fn destination_access_token(&self) -> std::option::Option<&str> {
        self.destination_access_token.as_deref()
    }
}
impl std::fmt::Debug for OpenTunnelOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("OpenTunnelOutput");
        formatter.field("tunnel_id", &self.tunnel_id);
        formatter.field("tunnel_arn", &self.tunnel_arn);
        formatter.field("source_access_token", &"*** Sensitive Data Redacted ***");
        formatter.field(
            "destination_access_token",
            &"*** Sensitive Data Redacted ***",
        );
        formatter.finish()
    }
}
/// See [`OpenTunnelOutput`](crate::output::OpenTunnelOutput).
pub mod open_tunnel_output {

    /// A builder for [`OpenTunnelOutput`](crate::output::OpenTunnelOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default)]
    pub struct Builder {
        pub(crate) tunnel_id: std::option::Option<std::string::String>,
        pub(crate) tunnel_arn: std::option::Option<std::string::String>,
        pub(crate) source_access_token: std::option::Option<std::string::String>,
        pub(crate) destination_access_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>A unique alpha-numeric tunnel ID.</p>
        pub fn tunnel_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.tunnel_id = Some(input.into());
            self
        }
        /// <p>A unique alpha-numeric tunnel ID.</p>
        pub fn set_tunnel_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.tunnel_id = input;
            self
        }
        /// <p>The Amazon Resource Name for the tunnel.</p>
        pub fn tunnel_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.tunnel_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name for the tunnel.</p>
        pub fn set_tunnel_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.tunnel_arn = input;
            self
        }
        /// <p>The access token the source local proxy uses to connect to IoT Secure Tunneling.</p>
        pub fn source_access_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.source_access_token = Some(input.into());
            self
        }
        /// <p>The access token the source local proxy uses to connect to IoT Secure Tunneling.</p>
        pub fn set_source_access_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.source_access_token = input;
            self
        }
        /// <p>The access token the destination local proxy uses to connect to IoT Secure Tunneling.</p>
        pub fn destination_access_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.destination_access_token = Some(input.into());
            self
        }
        /// <p>The access token the destination local proxy uses to connect to IoT Secure Tunneling.</p>
        pub fn set_destination_access_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.destination_access_token = input;
            self
        }
        /// Consumes the builder and constructs a [`OpenTunnelOutput`](crate::output::OpenTunnelOutput).
        pub fn build(self) -> crate::output::OpenTunnelOutput {
            crate::output::OpenTunnelOutput {
                tunnel_id: self.tunnel_id,
                tunnel_arn: self.tunnel_arn,
                source_access_token: self.source_access_token,
                destination_access_token: self.destination_access_token,
            }
        }
    }
    impl std::fmt::Debug for Builder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut formatter = f.debug_struct("Builder");
            formatter.field("tunnel_id", &self.tunnel_id);
            formatter.field("tunnel_arn", &self.tunnel_arn);
            formatter.field("source_access_token", &"*** Sensitive Data Redacted ***");
            formatter.field(
                "destination_access_token",
                &"*** Sensitive Data Redacted ***",
            );
            formatter.finish()
        }
    }
}
impl OpenTunnelOutput {
    /// Creates a new builder-style object to manufacture [`OpenTunnelOutput`](crate::output::OpenTunnelOutput).
    pub fn builder() -> crate::output::open_tunnel_output::Builder {
        crate::output::open_tunnel_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListTunnelsOutput {
    /// <p>A short description of the tunnels in an Amazon Web Services account.</p>
    #[doc(hidden)]
    pub tunnel_summaries: std::option::Option<std::vec::Vec<crate::model::TunnelSummary>>,
    /// <p>The token to use to get the next set of results, or null if there are no additional results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListTunnelsOutput {
    /// <p>A short description of the tunnels in an Amazon Web Services account.</p>
    pub fn tunnel_summaries(&self) -> std::option::Option<&[crate::model::TunnelSummary]> {
        self.tunnel_summaries.as_deref()
    }
    /// <p>The token to use to get the next set of results, or null if there are no additional results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
/// See [`ListTunnelsOutput`](crate::output::ListTunnelsOutput).
pub mod list_tunnels_output {

    /// A builder for [`ListTunnelsOutput`](crate::output::ListTunnelsOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) tunnel_summaries:
            std::option::Option<std::vec::Vec<crate::model::TunnelSummary>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `tunnel_summaries`.
        ///
        /// To override the contents of this collection use [`set_tunnel_summaries`](Self::set_tunnel_summaries).
        ///
        /// <p>A short description of the tunnels in an Amazon Web Services account.</p>
        pub fn tunnel_summaries(mut self, input: crate::model::TunnelSummary) -> Self {
            let mut v = self.tunnel_summaries.unwrap_or_default();
            v.push(input);
            self.tunnel_summaries = Some(v);
            self
        }
        /// <p>A short description of the tunnels in an Amazon Web Services account.</p>
        pub fn set_tunnel_summaries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::TunnelSummary>>,
        ) -> Self {
            self.tunnel_summaries = input;
            self
        }
        /// <p>The token to use to get the next set of results, or null if there are no additional results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The token to use to get the next set of results, or null if there are no additional results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListTunnelsOutput`](crate::output::ListTunnelsOutput).
        pub fn build(self) -> crate::output::ListTunnelsOutput {
            crate::output::ListTunnelsOutput {
                tunnel_summaries: self.tunnel_summaries,
                next_token: self.next_token,
            }
        }
    }
}
impl ListTunnelsOutput {
    /// Creates a new builder-style object to manufacture [`ListTunnelsOutput`](crate::output::ListTunnelsOutput).
    pub fn builder() -> crate::output::list_tunnels_output::Builder {
        crate::output::list_tunnels_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListTagsForResourceOutput {
    /// <p>The tags for the specified resource.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
}
impl ListTagsForResourceOutput {
    /// <p>The tags for the specified resource.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::model::Tag]> {
        self.tags.as_deref()
    }
}
/// See [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
pub mod list_tags_for_resource_output {

    /// A builder for [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
    }
    impl Builder {
        /// Appends an item to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>The tags for the specified resource.</p>
        pub fn tags(mut self, input: crate::model::Tag) -> Self {
            let mut v = self.tags.unwrap_or_default();
            v.push(input);
            self.tags = Some(v);
            self
        }
        /// <p>The tags for the specified resource.</p>
        pub fn set_tags(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Tag>>,
        ) -> Self {
            self.tags = input;
            self
        }
        /// Consumes the builder and constructs a [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
        pub fn build(self) -> crate::output::ListTagsForResourceOutput {
            crate::output::ListTagsForResourceOutput { tags: self.tags }
        }
    }
}
impl ListTagsForResourceOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
    pub fn builder() -> crate::output::list_tags_for_resource_output::Builder {
        crate::output::list_tags_for_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeTunnelOutput {
    /// <p>The tunnel being described.</p>
    #[doc(hidden)]
    pub tunnel: std::option::Option<crate::model::Tunnel>,
}
impl DescribeTunnelOutput {
    /// <p>The tunnel being described.</p>
    pub fn tunnel(&self) -> std::option::Option<&crate::model::Tunnel> {
        self.tunnel.as_ref()
    }
}
/// See [`DescribeTunnelOutput`](crate::output::DescribeTunnelOutput).
pub mod describe_tunnel_output {

    /// A builder for [`DescribeTunnelOutput`](crate::output::DescribeTunnelOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) tunnel: std::option::Option<crate::model::Tunnel>,
    }
    impl Builder {
        /// <p>The tunnel being described.</p>
        pub fn tunnel(mut self, input: crate::model::Tunnel) -> Self {
            self.tunnel = Some(input);
            self
        }
        /// <p>The tunnel being described.</p>
        pub fn set_tunnel(mut self, input: std::option::Option<crate::model::Tunnel>) -> Self {
            self.tunnel = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeTunnelOutput`](crate::output::DescribeTunnelOutput).
        pub fn build(self) -> crate::output::DescribeTunnelOutput {
            crate::output::DescribeTunnelOutput {
                tunnel: self.tunnel,
            }
        }
    }
}
impl DescribeTunnelOutput {
    /// Creates a new builder-style object to manufacture [`DescribeTunnelOutput`](crate::output::DescribeTunnelOutput).
    pub fn builder() -> crate::output::describe_tunnel_output::Builder {
        crate::output::describe_tunnel_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CloseTunnelOutput {}
/// See [`CloseTunnelOutput`](crate::output::CloseTunnelOutput).
pub mod close_tunnel_output {

    /// A builder for [`CloseTunnelOutput`](crate::output::CloseTunnelOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`CloseTunnelOutput`](crate::output::CloseTunnelOutput).
        pub fn build(self) -> crate::output::CloseTunnelOutput {
            crate::output::CloseTunnelOutput {}
        }
    }
}
impl CloseTunnelOutput {
    /// Creates a new builder-style object to manufacture [`CloseTunnelOutput`](crate::output::CloseTunnelOutput).
    pub fn builder() -> crate::output::close_tunnel_output::Builder {
        crate::output::close_tunnel_output::Builder::default()
    }
}
