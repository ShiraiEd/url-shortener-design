# url-shortner

A web application built with Rapina.

## Getting started

```bash
rapina dev
```

## Routes

- `GET /` — Hello world
- `GET /health` — Health check

# Problems so far

``` rapina add resource urls id:i64 short_code:string long_url:text created_at:datetime expires_at:datetime click_count:i64 ```
Created i64 instead of bigint
urls -> urlss

## after 
mod urlss;
mod entity;
mod migrations;
->
field 'created_at' is auto-generated. Use #[timestamps(none)] or #[timestamps(updated_at)] to declare it manually

added #[timestamps(none)] to entity

in #[post]
first declared short_code: Set(String::new()), inserted id into short_code, them updated short_code with base62 encoded id.
Maybe there is a more efficient way to do this.

