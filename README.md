# A tool for deriving the Ed25519 public key for a libp2p PeerId

This utility program allows to derive the public key for a `libp2p` [PeerId](https://docs.rs/libp2p/0.44.0/libp2p/core/struct.PeerId.html) encoded as a _base58_ string.
Such "inverse" conversion is possible due to the fact that the [Multihash](https://docs.rs/multihash/0.16.2/multihash/struct.MultihashGeneric.html) hashing algorithm used for creating the `PeerId` based on a valid `Ed25519` public key is, in fact, the [Identity](https://docs.rs/multihash/0.16.2/multihash/struct.IdentityHasher.html) transform.

<br>

## Building the app

1. Navigate to the project root directory:

    ```bash
    $ cd peerid2pubkey
    ```

2. Run `cargo build` command:

    ```bash
    $ cargo build --release
    ```

3. Copy the binary to some place on your `$PATH`:

    ```bash
    $ cp ./target/release/peerid2pubkey /usr/local/bin/
    ```
<br>

## Running the program

The program accepts two types of inputs:
- a file with _base58_ encoded peer IDs (one per line), or
- an arbitrary number of base58 encoded peer IDs as command line arguments.

The output is written into the standard output. Each peer ID from the input will have a corresponding line in the output. Mixing files and actual peer IDs is also possible, however, extra care is required to match inputs to the outputs.

In case of an error the `"invalid input"` message will be written in place of the expected respective output.

<br>

## Examples

1. Let's assume we've got a file with peer IDs:

    ```bash
    $ cat ./peer_ids.txt

    12D3KooWFafgzRvxrqYnyjdcYGSmNpNyDFry4SuCWmMp6XNsfL3X
    12D3KooWACewHVAnYNNhUdLe3vtrKVtcSocTkjJkMVu1KTY5gABU
    12D3KooWC7pcr6x5pRAFUobdnHSjd6s3XybjSZh4wsHuBbXFVU1A
    12D3KooWL8FKzv5GJvRBcQHzuKemJ34Bjm6YcsVw6XhDYJ7ygNdz
    12D3KooWEXeXuuB1UaxkpT6kpf2NqrD5hPTG5QnNLxQpzvy5wF2z
    12D3KooWQy5YY9mvGyChZVGLorz1MoTATDKPkCv77Rcn46uHDZwm
    12D3KooWQKi5zQUd6xLRnzLfieYzrB21PEaaLa9jigqRADbcnpj5
    12D3KooWHA3yqsEjr18rYfsaeBW8x6XeuoEHtdRtXsRHqHArJqCw
    12D3KooWMKmo8eyVYG2173XSBoCW2CZ16ahp8NDjZpA5r6WMGdsp
    12D3KooWEMkevzvkc5soWRE9h1xQ2UZdH6qhLg2NvEiCdZfaBYt9
    ```
    Then running the program with that file as an argument will produce
    ```bash
    $ peerid2pubkey ./peer_ids.txt

    0x55a3cd45bacceeb144effe3bff4cc69b89007c59ae32b47a8cc5102eabbfde6e
    0x05b5bcc460589b0a1ff4848d5fb7a5d69bb33edcbcacce8d3e24f20297ddcedb
    0x2230279f57328f33d6fa0ee56a6feb8181f9249019e92a454a7883d57ab9b855
    0x992996ddffed2b251d27ec16c4d5f2036c957f03c6d153ff7e61551aa027016d
    0x4602109779ab7a4ce796a3a0356274f1bb07ad5396a3824e8c9e68b4ecd4b6db
    0xe11abbec741914609672b843ac905604a70600ac78a36b946cc877a85c27ea08
    0xd787fb590c97bf3be2226407d7d1718b5a5c7b16718ff6cf9c938eb91bb4ba1c
    0x6d0cdc0736eb5a8908cf9d9436a9b536f812679e23c3218d67b64bc6ff495ccc
    0xaaf91d7f3cbb505c4e5a996bddfff303742d2cf5d4ed7c24b92cfe2c827f510b
    0x43792d82b8af783230049d0561f072d74296f59351c8e6c764525841e0b0b052
    ```

2. Alternatively, if we only have a handful of peer IDs we want to convert into Ed25519 public keys, we can run
    ```bash
    $ peerid2pubkey 12D3KooWG8cXpYEiYKZGQR1hXMuBJRhQAneTau1XHDa2GmNQYiUE 12D3KooWATHfadjvaH4tkQyXbxPZH5HB6saUiWm4UvwaA4HXJmMW

    0x5dd2d11cd4bfc3a63f8e715a2a6aafc0b9c4e04172e12dc02d2aa81bca85d9d7
    0x097566092a3f72f1d47e5f255b2e0ae4de557ca41d84ab8a0f06224b29650f9d
    ```
