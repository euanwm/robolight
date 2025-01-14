# The RoboLight QuickHack

__Problem__: My partner doesn't know when i'm in meetings as my office is on a different floor

__Solution__: Install an "on-air" light

## The Plan
- Buy a light bulb
- ???
- Profit

## The Implementation
A rough timeline of maybe an hour of work and then some tweaking
- Buy a Tapo bulb (because there's an API on GitHub)
- Write some Rust code to control the bulb
- Write some more Rust to detect when I'm in a meeting
- GCal was dumb
- Zoom was dumb
- Slack was dumb
- Seriously, every modern app is typescript wrapped in more typescript built on top of a webview
- Used the uvcvideo kernel process as an indicator through the ps -aux command
- Done

## The Summary
- I have a light that changes colour when I'm in a meeting
- It's a bit hacky but it works
- I'm happy

![RoboLight in action](./light.gif)
