pub mod Twitter {
    use twitter_v2::TwitterApi;
    use twitter_v2::authorization::{Oauth2Token,BearerToken};
    use serde_json;

    pub fn authorise() -> TwitterApi<BearerToken> {
        println!("{}", std::env::var("BEARER_TOKEN").unwrap());
        let auth = BearerToken::new(std::env::var("BEARER_TOKEN").unwrap());
        println!("Successfully authorised.");
        TwitterApi::new(auth)
    }

    pub fn auth2authorise() {
        let auth: Oauth2Token = todo!();
        let my_followers = TwitterApi::new(auth);
    }

}