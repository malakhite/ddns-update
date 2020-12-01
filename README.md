# ddns-update
A small application to update dynamic IP addresses (for Cloudflare, at least for now). Written in Rust.

## Configuration
All of the configuration happens in environment variables. The application expects the following variables to be set:
* `CLOUDFLARE_TOKEN`: Your API token from Cloudflare
* `CLOUDFLARE_ZONE`: The zone id of the domain you're updating
* `CLOUDFLARE_DNS_RECORD`: The id of the DNS record you're updating
* `POLL_INTERVAL`: How often you want to check for an updated IP address

The application will use a `.env` file if it exists.