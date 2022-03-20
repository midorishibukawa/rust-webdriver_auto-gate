<div align="center">
  <h2>auto-gate</h2>
  <p>A web automation project made with Rust and the <a href="https://github.com/stevepryde/thirtyfour">Thirtyfour</a> Selenium/WebDriver library.<p>
</div>

<h2>About</h2>
<p>This program automates the task of looking for items with a wrong code in a list, and fixing them by looking up the correct codes in a csv file.</p>
<p>It depends on the following crates:</p>
<ul>
  <li><code>thirfyfour</code>: to interact with the WebDriver server</li>
  <li><code>tokio</code>: to provide asynchronous functionalities</li>
  <li><code>csv</code>: to provide csv parsing with serde support</li>
  <li><code>serde</code> and <code>serde_json</code>: to provide deserialization of csv data into Rust structs</li>
 </ul>

<h2>Requirements</h2>
<h3>Rust Toolchain</h3>
<p>To run the program, you'll need the standard Rust toolchain, including <code>rustup</code>, <code>rustc</code>, and <code>cargo</code>.</p>
<p>You can follow <a href="https://www.rust-lang.org/tools/install">these instructions</a> to install it.</p>
<h3>Selenium or a standalone Chrome WebDriver</h3>
<p>You'll also need Selenium, or a Chrome WebDriver (<code>chromedriver</code>) that matches your Chrome version.</p>
<p>If you're using <code>chromedriver</code> directly, you need to run it on port 4444:</p>
<p><code>chromedriver --port=4444</code></p>
<p>If you're using Selenium, please follow <a href="https://github.com/stevepryde/thirtyfour#running-against-selenium">Thirthyfour's official instructions to run against it</a>.</p>

<h2>Usage</h2>
<p>You can start the automation by running:</p>
<p><code>cargo run</code></p>
