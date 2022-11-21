# hurlurl, a load balancing link shortener

Hurlurl can take a list of urls and create one url that randomly redirects to one of them.

This could be useful, if you have a large group of people, e.g. on a discord server or a twitch stream and you want to send them to different instances of a game or an online whiteboard for example. 
I made hurlurl when I needed to send people to different boards on https://hellopaint.io, for an event we did. 

I guess you could call hurlurl a _social load balancer_ because it's used to load balance people and not http requests. 
Although hurlurl could also be used for traditional load balancing. 

Give it a try on https://hurlurl.com/


## Development

`web` contains the frontend, written in Rust with Yew.
`urllb` contains the backend, written in Rust with Diesel and axum.

```bash
# Install
yarn install


# Start Postgres
docker compose up -d

# Run the following commands in separate terminals
# The order is important (otherwise there will be missing files)

# Watch tailwind css changes
yarn tailwind

# Start frontend
yarn start

# Start backend
yarn start:backend
```
