// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread.

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    async fn hello_world() -> &'static str {
        "hello world"
    }
    
    #[test]
    fn test_hello() {
        let hello_world_future = hello_world();
        assert_eq!(block_on(hello_world_future), "hello world"); 
            // block_on runs the `future` and "hello, world" is returned
    }

    async fn learn_song() -> &'static str {
        "learned song"
    }

    async fn sing_song(song: &str) -> String {
        format!("{}, {}",song, "singing song")
    }

    async fn learn_and_sing() {
        let song = learn_song().await;
        sing_song(song).await;
    }

    async fn dance() -> &'static str {
        "dancing"
    }

    async fn async_main()  {
        let f1 = learn_and_sing();
        let f2 = dance();
        futures::join!(f1, f2);

    }

    #[test]
    fn multiple_units() {
        async_main();
        // assert_eq!(f1, "dance");
    }
}