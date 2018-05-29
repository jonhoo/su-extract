StumbleUpon is [going
away](https://medium.com/@gc/su-is-moving-to-mix-c2c3bff037a5) on June
30th 2018, and all your data will also go away with
it. If you, like me, have many fond memories of very late nights where
you just had to Stumble *one more time*, then this might make you sad.
What if you want to go on a nostalgia trip back to those old days?
You can technically [have your likes
transferred](http://help.stumbleupon.com/customer/en/portal/articles/2908172-transitioning-from-stumbleupon-to-mix)
to "Mix", the service that's taking over StumbleUpon, but that seems
like more hassle than it's worth. And besides, you don't even really get
your data that way.

So, I wrote a tool that fetches all the URLs that you have liked, and
stores them (along with the page's title) in CSV format. So you can
import them into, well, wherever.

Here's what you have to do

 1. `git clone https://github.com/jonhoo/su-extract.git`
 2. `cd su-extract && cargo run` (you'll need [Rust](https://www.rust-lang.org/en-US/install.html) installed).
 3. Open Google Chrome (Chromium should also work)
 4. Open the Network inspector (Ctrl+Shift+I, select the "Network" tab)
 5. Go to your StumbleUpon profile while logged in
 6. Put `all?userid` in the filter box in the top left
 7. Right click the one row and click Copy -> Copy as cURL
 8. Paste into the terminal where you did `cargo run`.

The program should show a little progress bar that displays how many
likes have been fetched so far, as well as much time remains. At the
end, all of your likes will be available in `likes.csv`. Enjoy!
