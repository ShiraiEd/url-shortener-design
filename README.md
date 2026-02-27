# url-shortener

A high-throughput URL shortening service. Converts long URLs into short codes and redirects via HTTP 302.

## Stack

- **Nginx** — load balancer, rate limiting, SSL termination
- **App Servers** — 3 stateless instances (round robin)
- **Redis** — cache layer, click counters
- **Postgres** — source of truth (Primary + Replica)
