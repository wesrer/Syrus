# Module `Messages`

## Code reading guide

You should start by looking at the file `message_traits.rs`. These traits
represent logical, high level concepts that are implemented by each of the
messages that can be received during a block transfer exchange.

### `message_traits.rs`

- The trait `Encode` has a single function `encode_to_bytes() -> Result<BytesMut, Errors>`
  - This function encodes a *message* (that the program has constructed and is ready to send out)
    to a byte stream that can then be sent across a network.

- The trait `Decode` has a single function `decode_from(buffer: &mut BytesMut) -> Result<Self, Error>`.
  - As is clear from the definition, it takes a byte stream and attempts to *project*
    it into the message type that is implementing it.
  - Because this is a *projection* operation, it can fail. If it fails, we return an error.
  - It comes with a default implementation because except for a few message types, we
    can directly feed the byte stream received into the structure generated from the
    specified protocol buffer by `prost`.
      - (Please check `prost` documentation on more
        details about how it transforms protocol buffers into rust structures.
        You might also want to have a look at the protocol buffer itself, which
        is at `resources/BEP.proto`)

- The trait `Utils` gives some common utility functions that are useful to across messages.
  - `verify_len()`
    - Makes sure that the bytestream containing the message is exactly the same size as
      the message announced.
      If not, it returns an error.

The next file to take a look at is `hello.rs`, which is the first message that the devices
connectioned to each other send out, whether or not the connection is accepted or rejected.
This module is fairly straightforward, and should serve to give a good idea of how encodings
and decodings are going to be implemented for the various kinds of messages.

### `hello.rs`

- The first thing to notice is the import of the `Hello` trait from the module `block_exchange_protocol`.
  - This is not the name of the module we generate from `Prost` that contains
    the Rust traits generated from the protocol buffer `messages`.
  - We import only the `Hello` trait in this module.

- The next thing to study would be the `Encode` implementation. The code should
  be fairly trivial to understand

- Now, we can talk about the `Decode` trait which we do overwrite because we
  want to add a few guard clauses that check if the packet received is in the
  desired format. This code should also be fairly trivial to read.


### `header.rs`

### `messages.rs`

