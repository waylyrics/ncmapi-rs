<h1 align=center>ncmapi-rs</h1>

NetEase Cloud Music API for Rust.


### Usage

```toml
[dependencies]
ncmapi = "0.1"
tokio = { version = "1", features = ["full"] }
```

```rust
use ncmapi::NcmApi;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error:Error>> {
    let api = NcmApi::default();
    let resp = api.cloud_search("mota", None).await;

    let res = resp.unwrap().deserialize_to_implict();
    println!("{:#?}", res);

    Ok(())
}
```


### Document

Most of the functions are self documented. If there is some confusion about the params of a funtion requires, figure out [here](https://neteasecloudmusicapi.vercel.app)



### How it works

* api: export api functions.
* client:
    * takes an ApiRequst, process it into a Request by presenting it with header and encrypt the payload etc. And then send requests to the server, takes the response and then returns the ApiResponse back.
    * cache

### Status

```rust
// failures:
    api::tests::test_album_sub,
    api::tests::test_album_sublist,
    api::tests::test_artist_sub,
    api::tests::test_artist_sublist,
    api::tests::test_comment_create,
    api::tests::test_daily_signin,
    api::tests::test_fm_trash,
    api::tests::test_like,
    api::tests::test_likelist,
    api::tests::test_login_phone,
    api::tests::test_login_refresh,
    api::tests::test_recommend_resource,
    api::tests::test_recommend_songs,
    api::tests::test_user_cloud,
    api::tests::test_user_level,
    api::tests::test_user_record,
    api::tests::test_user_subcount,
    types::tests::test_de_artist_sublist,
    types::tests::test_de_playlist_detail,
    types::tests::test_de_recommended_playlists,
    types::tests::test_de_recommended_songs,
    types::tests::test_de_user_cloud,
```

### Contribute

If you think this package useful, please do make pull requests.

### License

[MIT](LICENSE)