# Google Hangouts JSON Parser

This is a little Rust library for parsing the JSON you get when you export your Google Hangouts
data using [Google Takeout](https://takeout.google.com/settings/takeout).

At the moment, all it does is parse the JSON into strongly-typed structs and enums that very
closely match the structure of the underlying JSON. Eventually there will be a more high-level
interface that makes it easier to actually use the parsed data.
