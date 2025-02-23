<!-- Keep this comment so the content is always included as a new paragraph -->
This constructor requires an [`OriginalRoomMessageEvent`] since it creates a permalink to
the previous message, for which the room ID is required. If you want to reply to an
[`OriginalSyncRoomMessageEvent`], you have to convert it first by calling
[`.into_full_event()`][crate::events::OriginalSyncMessageLikeEvent::into_full_event].

It is recommended to enable the `sanitize` feature when using this method as this will
clean up nested [rich reply fallbacks] in chains of replies. This uses [`sanitize_html()`]
internally, with [`RemoveReplyFallback::Yes`].

[rich reply fallbacks]: https://spec.matrix.org/v1.2/client-server-api/#fallbacks-for-rich-replies
