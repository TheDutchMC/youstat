# YouStat
YouTube Music Statistics Generator

## Installing
1. Install the [Rust toolchain](https://www.rust-lang.org/learn/get-started)
2. `cargo install youstat

## Usage
1. Go to [Google Takeout](https://takeout.google.com)
2. Create a new export, you'll want to include `My Activity`, which itself should include `YouTube`. The activity record format should be `JSON`
3. Wait for the Takeout to complete, then extract the generated ZIP file
4. Create a new project in the [Google developers console](https://console.developers.google.com), create a new API token and enable the `YouTube Data API v3`
5. Lastly, generate your statistics: `youstat -i <Path to the generated My Activity.json file in the extracted ZIP> -k <Your API token>`
>Note: Optionally you can specify `-y` and `-m` too, use `-h` for more details on those

## Licence
YouStat is licenced under the [MIT license](LICENSE)