use futures::executor::block_on;

fn main() {
    let future = do_something();
    block_on(future);

    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());

    block_on(async_main());
}

async fn do_something() {
    println!("hi, this is do_something function");
}

async fn learn_song<'a>() -> &'a str {
    "We will rock you"
}

async fn sing_song(song: &str) {
    println!("singing \"{}\"", song);
}

async fn dance() {
    println!("dancing");
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let ls_f = learn_and_sing();
    let d_f = dance();
    futures::join!(ls_f, d_f);
}
