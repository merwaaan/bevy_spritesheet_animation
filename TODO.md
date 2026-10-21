# Make events optional

Events are always emitted, even when users never use them.
This becomes costly when many sprites are spawned.
Put them behind a feature flag or add a runtime option to toggle them.

# Optimize playback

There's currently a lot of allocations during playback (esp. for events).
Use a single storage for all sprites instead of allocating one at each sprite update.
