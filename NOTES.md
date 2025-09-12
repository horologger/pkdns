dig @192.168.1.1 -p 5353 +tcp example.com A

cat ~/.pkdns/pkdns.toml

export RUST_LOG=INFO

export RUST_LOG=DEBUG

target/debug/pkdns-cli --help Usage: pkdns-cli [COMMAND]

Commands: publish Publish pkarr dns records. resolve Resolve pkarr dns records. generate Generate a new zbase32 pkarr seed publickey Derive the public key from the seed. help Print this message or the help of the given subcommand(s)

Options: -h, --help Print help -V, --version Print version

https://medium.com/pubky/how-to-host-a-public-key-domain-website-v0-6-0-ubuntu-24-04-57e6f2cb6f77

target/debug/pkdns-cli generate > ~/.pkdns/seed.txt

target/debug/pkdns-cli publickey ~/.pkdns/seed.txt xcttw1teuudse4pw4c96rnhs9iodxgqzuqzyqdtcu3ustkxafnko
target/release/pkdns-cli publickey ~/.pkdns/seed.txt

echo "@ IN A 127.0.0.1" > ~/.pkdns/pkarr.zone
cat ~/.pkdns/pkarr.zone

echo "@ IN A 70.251.209.207" > ~/.pkdns/nostrops.zone
cat ~/.pkdns/nostrops.zone


target/debug/pkdns-cli publish ~/.pkdns/seed.txt ~/.pkdns/pkarr.zone
target/release/pkdns-cli publish ~/.pkdns/seed.txt ~/.pkdns/pkarr.zone
target/release/pkdns-cli publish ~/.pkdns/seed.txt ~/.pkdns/nostrops.zone

Packet xcttw1teuudse4pw4c96rnhs9iodxgqzuqzyqdtcu3ustkxafnko Name TTL Type Data
@ 300 A 127.0.0.1

1737748806174801 Successfully announced.

target/debug/pkdns-cli resolve xcttw1teuudse4pw4c96rnhs9iodxgqzuqzyqdtcu3ustkxafnko
target/release/pkdns-cli resolve xcttw1teuudse4pw4c96rnhs9iodxgqzuqzyqdtcu3ustkxafnko

http://xcttw1teuudse4pw4c96rnhs9iodxgqzuqzyqdtcu3ustkxafnko./

Resolve dns records of pk:xcttw1teuudse4pw4c96rnhs9iodxgqzuqzyqdtcu3ustkxafnko Packet xcttw1teuudse4pw4c96rnhs9iodxgqzuqzyqdtcu3ustkxafnko Name TTL Type Data
@ 300 A 127.0.0.1

Last updated at: 2025-01-24 20:00:06 UTC

target/debug/pkdns

Set breakpoint at async fn query_me_recursively server/src/resolution/dns_socket.rs line 262 server/src/resolution/pubkey_parser.rs line 14 parse_pkarr_uri

pkarr_resolver.rs Line 243 where is this called from??

dig xcttw1teuudse4pw4c96rnhs9iodxgqzuqzyqdtcu3ustkxafnko @127.0.0.1

Build for local testing
```
cargo build --target x86_64-unknown-linux-gnu --target armv7-unknown-linux-gnueabihf --release
cross build --release --target x86_64-unknown-linux-musl
cargo run --release
cargo run --package=pkdns --help

cargo build --release
screen -S pkdns -d -m ./target/release/pkdns -v ; screen -x pkdns

./target/release/pkdns-cli resolve 7fmjpcuuzf54hw18bsgi3zihzyh4awseeuq5tmojefaezjbd64cy
dig 7fmjpcuuzf54hw18bsgi3zihzyh4awseeuq5tmojefaezjbd64cy TXT @127.0.0.1
nslookup 7fmjpcuuzf54hw18bsgi3zihzyh4awseeuq5tmojefaezjbd64cy 127.0.0.1

nslookup -q=A 7fmjpcuuzf54hw18bsgi3zihzyh4awseeuq5tmojefaezjbd64cy 127.0.0.1
nslookup -q=TXT _text.7fmjpcuuzf54hw18bsgi3zihzyh4awseeuq5tmojefaezjbd64cy 127.0.0.1
nslookup -q=CNAME cname.7fmjpcuuzf54hw18bsgi3zihzyh4awseeuq5tmojefaezjbd64cy 127.0.0.1
```

# ./target/release/pkdns-cli resolve @space
# dig @space TXT @127.0.0.1
nslookup -q=A @space 127.0.0.1
nslookup -q=TXT @space 127.0.0.1

cargo build --target armv7-unknown-linux-gnueabihf --release
docker buildx build --platform linux/arm64 --tag horologger/pkdns:v0.7.0 --load .

cargo build --target x86_64-unknown-linux-gnu --release
docker buildx build --platform linux/amd64 --tag horologger/pkdns:v0.7.0 --load .
Build and push to docker hub (So we can install on Umbrel)
docker buildx build --platform linux/arm64,linux/amd64 --tag horologger/pkdns:v0.7.0 --output "type=registry" .
docker buildx build --platform linux/amd64 --tag horologger/pkdns:v0.7.0 --output "type=registry" .
docker run
-p 5353:53
--name pkdns
-it horologger/pkdns:v0.7.0

docker stop pkdns ; docker rm pkdns

docker exec -it pkdns /bin/bash


# Testing
lynx @nostrops
lynx @1984
lynx @space
lynx @test
lynx @local
lynx @nostr.com
