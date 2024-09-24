# AutoDoc Monitor

The goal of this monitor is to track the price of certain car parts and accessories on [AutoDoc](https://www.autodoc.nl/).

## Data fetching

To be able to monitor the prices of certain products, I had to find a way to fetch data from AutoDoc, so that I could transform it into Oculite's format.

### The AutoDoc website

My first instinct was to look at the AutoDoc website. After investigating the network requests sent, when on a product page, I unfortunately came to the conclusion I had to look elsewhere. It turned out that the website provided all of the product data statically in the HTML, and the price was not fetched using a back-end API.

<img alt="Static data embedded into the HTML", src="/@meta/images/autodoc/website.png">

I did not want to write a web scraper, as I felt like this would be unreliable and could break at any time. I wanted to find a more stable solution with a better user-experience. Ultimately, my goal was to allow users to able to paste a link to a product in Oculite, and it would automatically start monitoring the price.

### The AutoDoc mobile app

During a conversation with my good friend [Joey Li](https://github.com/einstein8612), who has extensive experience with web development, he suggested to me to look at the mobile app of AutoDoc, since most apps fetch their data using a back-end API. This was a great idea, and I started to investigate the network requests sent by the AutoDoc mobile app.

The goal was to find the API endpoint that the mobile app was using to fetch the product data. This meant capturing the network requests sent by the app, and analyzing the data that was being sent and received, so that I could replicate it myself.

#### Setting up the Android Emulator environment

It turns out that on IOS I had 3 options to start sniffing network traffic: Charles Proxy, which is a paid app; Jail-breaking my phone, which I did not want to do; or using a Mac, which I did not have. Since I knew AutoDoc had a mobile app for android as well, I decided to use an Android emulator in the hopes of being able to sniff the network traffic.

I downloaded [Android Studio](https://developer.android.com/studio) and created a new virtual device. Installing the AutoDoc app turned out to be a bit of a hassle. The AutoDoc app was not available in the Google Play Store for some reason, so I had to find the apk on the internet. Luckily, I managed to find the apk online, and installed it on the emulator by dragging and dropping the apk file.

#### Sniffing network traffic

I then started sniffing the network traffic using [HTTP Toolkit](https://httptoolkit.com/). This was a fairly easy process to set up, since HTTP Toolkit took care of all of the configuration for me. It turned out there was one slight issue however.

<img alt="HTTP Toolkit system trust is disabled", src="/@meta/images/autodoc/http_toolkit_app.png" height="25">

The issue was that the Android emulator did not (want to) install HTTP Toolkit's CA certificate as a system certificate, which meant that any apps who opted out of using user certificates would not work with HTTP Toolkit. To confirm this, I opened the AutoDoc app and like I suspected, the app kept loading indefinitely.

<img alt="AutoDoc app keeps loading indefinitely", src="/@meta/images/autodoc/autodoc_app_loading.png" height="25">

Checking out the HTTP Toolkit logs, showed that the certificates were being rejected.

<img alt="HTTP Toolkit logs showing certificate rejection", src="/@meta/images/autodoc/http_toolkit_no_trust.png">

After some investigation, the reason for this was the following, according to the HTTP Toolkit docs, "This (System Trust disabled) is common and unavoidable when using HTTP Toolkit on non-rooted devices or locked-down emulators such as the 'Google Play' official emulator builds.". It turns out that was using an emulator build where Google Play services were enabled.

So, the solution was to switch to a different emulator build, where Google Play services were not enabled. I opted for the "Android 12.0 ("S") | x86_64" build. I installed the AutoDoc app through the APK on this new emulator build, as well as enabled HTTP Toolkit for this emulator build and opened the AutoDoc app.

<img alt="AutoDoc app shows API requests", src="/@meta/images/autodoc/android_emulator.png">

It worked! The AutoDoc app was now showing API requests in HTTP Toolkit. I could now start analyzing the network requests sent by the AutoDoc app.

### Analyzing the network requests

HTTP Toolkit showed a bunch of network request being sent by the AutoDoc app. One particularly stood out to me. A request was being made to `apim.autodoc.de`. The descriptive name of the endpoint, `apim`, made me think that this was the API endpoint that the AutoDoc app was using to fetch the product data. I started analyzing the response body.

<img alt="AutoDoc API response", src="/@meta/images/autodoc/http_response_body.png">

The response body seems to contain all of the product data that I was able to see on the product page. This was great news! I could now analyzing this request to see how I could replicate it. Beware, the following screenshot will look like something straight out of a classified FBI report.

<img alt="AutoDoc API request", src="/@meta/images/autodoc/request_headers.png">

The request headers contained a lot of information. The only vital headers for a successful API query were the `api-project-country`, `api-project-lang`, `api-platform`, `api-version` and `api-token` headers. The `api-token` and `api-platform` are self-explanatory. After some experimenting I found out that `api-project-lang`, like the name suggests, controls the language of the API response; Changing it to 9, changed the response body to Dutch and changing it to 4 changed the response body to English. The `api-version` interestingly controlled the output of the response. When not included the name of the product used commas instead of `|`, but this is for the most part irrelevant to our goal. The `api-project-country` header was required to be set in the request, but was irrelevant to me since I had no intention of changing the country of the API response and I had all of the data I needed in the response body.

It seems like the query parameters were not required for a successful API query. The only thing needed was the final path parameter which seemed to correspond to the product ID. Conveniently, when viewing a product on the AutoDoc website this product ID was the last part of the URL as well: `https://www.autodoc.nl/ridex/7999203`. This meant that I could easily extract the product ID from the URL and use it in the API query.

The response returned by AutoDoc's API contained all of the product data I needed. The product name, the product price and even a CDN hosted link to the product image. This was super convenient for me! I could now start transforming this data into Oculite's format with minimal effort.

## Data transformation

The only thing the AutoDoc monitor had to do was, upon request, go through the config string provided by Oculite, extract the product IDs from the URLs and fetch the product data from AutoDoc's API. The fetched data would then be transformed into Oculite's format and returned to Oculite. Voilà! The AutoDoc monitor is done.
