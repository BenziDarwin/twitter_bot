pub mod twitter;
use twitter::Twitter::authorise;
use dotenv::dotenv;
use twitter_v2::query::TweetField;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let tweeter =  authorise();
    let data  = tweeter.get_tweet(1261326399320715264)
    .tweet_fields([TweetField::AuthorId, TweetField::CreatedAt])
    .send()
    .await.unwrap()
    .into_data()
    .expect("this tweet should exist");
    println!("{}",data.text);
}
