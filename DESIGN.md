<<<<<<< Updated upstream
1.Overview
Intent to do a url-shortener
teste
teste
=======
1.Overflow
-Intended to do a url shortener
-Transform a long-url to a short one
-And redirect the short-url to corresponding long-url

2.API design
-Endpoints
shorten endpoint - request: POST api/v1/shorten
	Body:{“url” : “www.example.com”....}
	
	response: Status code 201 (means created) 
	{“short_url”: “short/……..”}

redirection endpoint - request: GET”short/…..”

	response: Status code 301/302 (redirect)
	Header : 
	     location:  “www.example.com/…”

http status code:
201 if created 
400 if invalid

301 or 302: In first place, maybe a 302 would be more suitable temporary, because:
We might want to change the destination later

We may support expiration
We might do A/B testing or geo-based routing
We want accurate analytics (301 can get cached aggressively)

If we use 301, browsers may cache permanently and stop hitting our backend.
I’d only use 301 if:

Links are guaranteed immutable
No dynamic routing
We want maximum caching performance

301 might look good in first place, considering performance once links become immutable.But you can’t analise statistics which links are most accessed and if you might change something in the future, 301 would be a problem too.302 is more flexible, in other hand it higher server load and gets slightly higher latency.


3.Short ID generation
If I’m using auto-increment IDs + Base62, the idea is pretty straightforward.
The database generates a numeric ID for each new URL. Let’s say it gives me something like 1000001. Then I just encode that number into Base62 so it becomes shorter and URL-friendly, like 4c92.
So internally it’s still just a number — Base62 is only for presentation.
The nice part about this approach is that I don’t really worry about collisions. The database guarantees uniqueness because the ID is auto-incremented. There’s no retry logic, no randomness, no probability involved. It’s very simple and reliable.

The main tradeoff is that the IDs are predictable. Since they’re sequential, someone could technically enumerate them and try accessing nearby short URLs. Depending on the product, that might or might not matter.
Another limitation is in distributed systems. If multiple services need to generate IDs independently, you can’t just rely on a single auto-increment counter unless you centralize it or use something like a distributed ID generator.
But honestly, for a simple and centralized URL shortener, this approach is very practical. It’s clean, easy to reason about, and performs well.

5. Redirect Flow

When someone hits the short URL:
* Load balancer routes request
* Backend extracts short_id
* Lookup in cache or DB
* Validate expiration
* Increment analytics
* Return redirect
>>>>>>> Stashed changes
