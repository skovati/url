# url
very basic url shortner built with rust, tokio, warp, and sqlite

## installation
```sh
git clone https://github.com/skovati/url
cargo run
```

## usage
```sh
curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"url":"http://skovati.dev"}' \
  http://localhost:8080
```
returns
```sh
http://localhost:8080/ae7d
```
```sh
librewolf http://localhost:8080/ae7d
```
will now bring you to [skovati.dev](https://skovati.dev)
