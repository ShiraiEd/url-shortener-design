# url-shortener

A high-throughput URL shortening service. Converts long URLs into short codes and redirects via HTTP 302.

## Stack

- **Rapina** — Is simple and comes with all the rust benefits
- **In-memory-cache** — caches hot path GET:short_code
- **Sqlite** — Simpler for the V1

## how to run

1 -```cd``` into url-shortner
2 -```rapina dev``` to ru the localserver
3 -open another terminal
4 -use ```curl``` to interact with the api

examples :
- **Create shortcode** -> ```curl -X POST http://localhost:3000/api/v1/shorten   -H "Content-Type: application/json"   -d '{"long_url": "https://userapina.com/"}'```
- **List shortcodes** -> ```curl -X GET http://localhost:3000/api/v1/shorten```
- **Metadata** -> ```curl -v http://localhost:3000/api/v1/shorten/:short_code```
- **Delete** -> ```curl -X DELETE http://127.0.0.1:3000/api/v1/shorten/short_code```

## Implementation decisions

- 302 instead of 301 for click-counting
- For v1 still don't think that Postgres or Redis is needed, for now Sqlite and in-memory-cache works just fine

## What is missing

- for the next versions, I Think it needs a login an verification,
- the databases changes mentioned above,
- maybe a frontend UI for the login page, and one to use the api without curl 
