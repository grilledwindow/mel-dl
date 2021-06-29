# mel-dl
This project aims to automate the downloading of weekly files from my school's website [MeL](https://mel.np.edu.sg). It is somewhat working but there are still things to be worked on.

## Prerequisites
You need to have [Rust](https://www.rust-lang.org/tools/install), [Chromedriver](https://chromedriver.chromium.org/downloads) and a Chromium-based browser installed (I'm using [Brave](https://brave.com/download/)).

## Limitations
I realised that I can't specify the download locations without initialising a new `WebDriver`, which would open a new window. The workaround I thought of is to first download files to a temporary folder, which is `.mel-dl` in my case, and move the files into the modules' specified directory afterwards.

## Setup
Run the chromedriver:\
`chromedriver --port=4444`

Build and run (in a separate tab):\
`cargo run`
