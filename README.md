# Asterisk ARI Client
[![CI](https://github.com/jabber-tools/asterisk-ari-client-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/jabber-tools/asterisk-ari-client-rs/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/License-Apache-blue.svg)](LICENSE-APACHE)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![version](https://img.shields.io/crates/v/asterisk-ari-client-rs)](https://crates.io/crates/asterisk-ari-client-rs)
[![docs](https://docs.rs/asterisk-ari-client-rs/badge.svg)](https://docs.rs/asterisk-ari-client-rs)

Simple [Asterisk](https://www.asterisk.org/) library. Implements only small fraction of available Asterisk REST APIs known as [ARI](https://wiki.asterisk.org/wiki/pages/viewpage.action?pageId=29395573). Enables to connect to Asterisk websocket and listen for following events:

* stasis_start
* channel_dtmf_received
* channel_hangup_request
* stasis_end
* channel_talking_finished
* channel_talking_started
* channel_destroyed
* playback_started
* playback_finished
* channel_state_change
* channel_var_set
  
Apart from that following channels' operations are supported:

* answer
* play
* stop_play
* get_variable
* set_variable
* hangup
* continue_in_dialplan
* record

Supported recording API operations:

* stop_recording
* pause_recording
* unpause_recording
* mute_recording
* unmute_recording
* delete_recording

Supported application API operations:

* filter
* get
* list
* subscribe
* unsubscribe

This is by no means ready library. It is used for now on single purpose project and needs to be extended to support other ARI APIs. Pull requests welcome!

## License

Licensed under either Apache-2.0 or MIT license. 