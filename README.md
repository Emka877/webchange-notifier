## What's this?
I needed a tool that constantly tracks changes occurring on a webpage, so I did one.

I know there are existing services that do exactly this, but I have no control over it, and most of the time they become paid solutions after a while.


## Current state: WIP, working, must test

## Requirements
* A Pushover account
* Create a Pushover app

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
* The possibility to connect with other push notification services
* Being able to connect to many PN services at once
* The possibility to send a screenshot with the push notification (if the PN service allows it)
* Target multiple pages
* Different timeouts for each target web page
* The possibility to force the re-creation of a "base"
* Know what part(s) of the page changed
