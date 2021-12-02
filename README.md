## What's this?
I needed a tool that constantly tracks changes occurring on a webpage, so I did one.


## Current state: WIP

## How to use?
1. Modify the `conf/example_config.ron` file to your convenience.
   1. Rename `example_config.ron` to `config.ron`.
2. Run (as a daemon/service or not) on a server or on your computer.
3. It should now send your push notifications whenever a page changes.

## How does it work?
A first pass is made to retrieve the initial state of the webpage ("base"), then this base is used to check upon ulterior checks of the same webpage.

## Why Rust?
I know Rust would be taking a cannon to get the mosquito (Node or Python would perfectly do this job), but I'm currently trying to learn Rust and get accustomed to its ecosystem, that's why I chose this language.

## Future features
* The possibility to connect with another push notification service
* The possibility to send a screenshot with the push notification (if the PN service allows it)
* Different timeouts for each target web page
* The possibility to force the re-creation of a "base"
* Know what part(s) of the page changed