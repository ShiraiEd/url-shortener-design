# url-shortener

A high-throughput URL shortening service. Converts long URLs into short codes and redirects via HTTP 302.

## Stack

- **Nginx** — load balancer, rate limiting
- **App Servers** — 2 or more stateless instances (round robin)
- **Redis** — cache layer, click counters, sentinel
- **Postgres** — source of truth (Primary + Replica). PgBouncer
