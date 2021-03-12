## Anime scraper

This is an anime scraper that stores anime metadata and servers urls where
videos are stored. It uses sqlite as a backend.

Currently, it only scrapes monoschinos.com, a spanish anime streaming website.

It works by visiting each anime and comparing the html checksum against a
stored one to see if it was already scraped previously to avoid innecessary
work. An idea would be to process only new anime episodes but video servers are updated even in
very old episodes so we need to check them all anyways.
