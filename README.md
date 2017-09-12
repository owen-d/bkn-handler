### Bkn Handler for [Sharecrows](https://my.sharecro.ws)

This is an impression handler which sits on the domain our.sharecro.ws and handles incoming GET requests for our beacon resources. Routes take the form `our.sharecro.ws/bkn/$EDDYSTONE_UID`. The UID can be either the standard 16 byte composition of a 10 byte namespace and 6 bye identifier, or just the identifier (which we attach to our namespace).

Requests are classified as either passbys or user interactions, via referrer headers. Google proxies passby requests on behalf of users, & thus referrers should be empty. User interactions have our.sharecro.ws as referrers, force refreshing the page via `location.assign('')` or `http-equiv` html elements. Both passbys and interactions are logged to cassandra.


#### Future improvements
* favicon spoofing support via prefetching & [Data URIs](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URIs)
