extern crate tokio;
extern crate futures;
extern crate fantoccini;

fn main() {
    use fantoccini::{Client, Locator};
    use futures::future::Future;
    let c = Client::new("http://localhost:4444");
   
    // let's set up the sequence of steps we want the browser to take
    tokio::run(
        c
            .map_err(|e| {
                unimplemented!("failed to connect to WebDriver: {:?}", e)
            })
            .and_then(|c| {
                // first, go to the Wikipedia page for Foobar
                c.goto("https://en.wikipedia.org/wiki/Foobar")
            })
            .and_then(|mut c| c.current_url().map(move |url| (c, url)))
            .and_then(|(mut c, url)| {
                assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foobar");
                // click "Foo (disambiguation)"
                c.find(Locator::Css(".mw-disambig"))
            })
            .and_then(|e| e.click())
            .and_then(|mut c| {
                // click "Foo Lake"
                c.find(Locator::LinkText("Foo Lake"))
            })
            .and_then(|e| e.click())
            .and_then(|mut c| c.current_url())
            .and_then(|url| {
                assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foo_Lake");
                Ok(())
            })
            .map_err(|e| {
                panic!("a WebDriver command failed: {:?}", e);
            })
    );
}
