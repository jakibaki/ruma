//! `POST /_matrix/client/*/rooms/{roomId}/receipt/{receiptType}/{eventId}`

pub mod v3 {
    //! `/v3/` ([spec])
    //!
    //! [spec]: https://spec.matrix.org/v1.2/client-server-api/#post_matrixclientv3roomsroomidreceiptreceipttypeeventid

    use ruma_common::{
        api::ruma_api,
        serde::{OrdAsRefStr, PartialEqAsRefStr, PartialOrdAsRefStr, StringEnum},
        EventId, RoomId,
    };

    use crate::PrivOwnedStr;

    ruma_api! {
        metadata: {
            description: "Send a receipt event to a room.",
            method: POST,
            name: "create_receipt",
            r0_path: "/_matrix/client/r0/rooms/:room_id/receipt/:receipt_type/:event_id",
            stable_path: "/_matrix/client/v3/rooms/:room_id/receipt/:receipt_type/:event_id",
            rate_limited: true,
            authentication: AccessToken,
            added: 1.0,
        }

        request: {
            /// The room in which to send the event.
            #[ruma_api(path)]
            pub room_id: &'a RoomId,

            /// The type of receipt to send.
            #[ruma_api(path)]
            pub receipt_type: ReceiptType,

            /// The event ID to acknowledge up to.
            #[ruma_api(path)]
            pub event_id: &'a EventId,
        }

        #[derive(Default)]
        response: {}

        error: crate::Error
    }

    impl<'a> Request<'a> {
        /// Creates a new `Request` with the given room ID, receipt type and event ID.
        pub fn new(room_id: &'a RoomId, receipt_type: ReceiptType, event_id: &'a EventId) -> Self {
            Self { room_id, receipt_type, event_id }
        }
    }

    impl Response {
        /// Creates an empty `Response`.
        pub fn new() -> Self {
            Self {}
        }
    }

    /// The type of receipt.
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/doc/string_enum.md"))]
    #[derive(Clone, Debug, PartialOrdAsRefStr, OrdAsRefStr, PartialEqAsRefStr, Eq, StringEnum)]
    #[non_exhaustive]
    pub enum ReceiptType {
        /// A [public read receipt].
        ///
        /// Indicates that the given event has been presented to the user.
        ///
        /// This receipt is federated to other users.
        ///
        /// [public read receipt]: https://spec.matrix.org/v1.3/client-server-api/#receipts
        #[ruma_enum(rename = "m.read")]
        Read,

        /// A [private read receipt].
        ///
        /// Indicates that the given event has been presented to the user.
        ///
        /// This read receipt is not federated so only the user and their homeserver
        /// are aware of it.
        ///
        /// [private read receipt]: https://github.com/matrix-org/matrix-spec-proposals/pull/2285
        #[cfg(feature = "unstable-msc2285")]
        #[ruma_enum(rename = "org.matrix.msc2285.read.private", alias = "m.read.private")]
        ReadPrivate,

        /// A [fully read marker].
        ///
        /// Indicates that the given event has been read by the user.
        ///
        /// This is actually not a receipt, but a piece of room account data. It is
        /// provided here for convenience.
        ///
        /// [fully read marker]: https://spec.matrix.org/v1.3/client-server-api/#fully-read-markers
        #[cfg(feature = "unstable-msc2285")]
        #[ruma_enum(rename = "m.fully_read")]
        FullyRead,

        #[doc(hidden)]
        _Custom(PrivOwnedStr),
    }
}
