# Prophesizer

[![prophesizer](https://img.shields.io/badge/GitHub-prophesizer-green?style=plastic)](https://www.github.com/Tylerr-D/prophesizer)

> for all your prophesizing dreams

---

## Features

*   **Random outputs**: Gives you random outputs based on inputs
*   **Daily Prophecy**: Gives you a new prophecy every day
*   **Stats**: Shows your most inputted words
*   **Diagnose**: Diagnoses and gives a report of your word
*   **Karma**: Shows your word's karma

## Stack

*   **Frontend:**   Terminal
*   **Backend:**    Rust
*   **Database:**   json
*   **Styling:**    Vibez

---

## Getting Started

Follow these simple steps to get a local copy up and running.

### Prerequisites

None (Unless you want to build it from source)

### How to get this for yourselves:

**Download it**

Get the latest release from [GitHub](https://github.com/Tylerr-D/prophesizer/releases)

>**Note:** The name of the executable will be "prophesizer-*", where * is the version number, and build version,
> remember to type the full name when executing like ```./prophesizer-* -V``` , or rename it from "prophesizer-*" to "prophesizer"

> If you downloaded, most likely it is in the downloads directory,
> so either move it to the home directory (/home/user/) or run ```cd ~/Downloads``` before
> doing ```./prophesizer```


Or download from command line, like this:


#### Download (Linux only)

```shell
curl -L https://github.com/Tylerr-D/prophesizer/releases/download/Release/prophesizer-0.1.0-x86_64-Linux -o prophesizer
chmod +x prophesizer
```

> Always check what you are running, don't run random commands you find on the internet.

Done!, add to path to run anywhere or run from home like:
```shell
./prophesizer -V
```

## Building from Source

(linux only)

1. **Pre-requisites:**  
   Need to install Rust


2. **Clone the repository:**
   ```shell
   git clone https://github.com/Tylerr-D/prophesizer.git
   cd prophesizer
   ```


3. **Build**:
   Probably just run:

   ```shell
   cargo build --release
   ```


4. **Done!:**  
   Now test the binary with:
   ```shell
   ./prophesizer -V
   ```

## Contributors
*   **[![Amaan](https://img.shields.io/badge/GitHub-MiniGun1239-orange?style=plastic)](https://www.github.com/MiniGun1239)**
*   **[![Ruster](https://img.shields.io/badge/GitHub-TylerrD-orange?style=plastic)](https://www.github.com/Tylerr-D)**

> Coded and tested in Ubuntu and Arch Linux, should work in any distro.

## Help
```shell
Usage: prophesizer [OPTIONS] [INPUT]

Arguments:
  [INPUT]  input any word

Options:
  -s, --stats                shows stats
  -d, --daily                shows daily prophecy
      --history <HISTORY>    shows last words you fed to the machine, takes input mode to sort
  -g, --diagnose <DIAGNOSE>  gives a medical report for a word
  -k, --karma                shows karma of every word you have fed the machine
  -h, --help                 Print help (see more with '--help')
  -V, --version              Print version
```
