# xtream2m3u
Create a XMLTV Guide for NFL Football games.\
It will use your Xtream codes to find the NFL 01 to NFL 16 \
channels and use the names from those channels to create\
the EPG.

# Installing

## Install Rust

### Linux
To install rust:\
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh\
This will install everything you need to use Rust

You will also need a few additional packages as OpenSSL is used.

#### Arch Linux
$ sudo pacman -S pkgconf openssl gcc

#### Debian and Ubuntu
$ sudo apt-get install pkg-config libssl-dev gcc

#### Fedora
$ sudo dnf install pkgconf perl-FindBin perl-IPC-Cmd openssl-devel gcc

#### Alpine Linux
$ apk add pkgconf openssl-dev gcc

#### openSUSE
$ sudo zypper in libopenssl-devel gcc


### Windows or Mac

Go to https://www.rust-lan.org/tools/install and follow the instructions.

#### Windows Executable for WIn10/11

If you do not want to install Rust you can download this instead:\
https://github.com/bmillham/xtream2m3u/releases/download/V0.1.5/xtream2m3u.zip

## Install nfl-epg

To install nfl-epg, clone this to your projects directory:\
mkdir -p projects\
cd projects\
git clone https://github.com/bmillham/nfl-epg

The project is now in projects/nfl-epg

cd nfl-epg

Everything from here on is done in the nfl-epg directory.

# Options
+ -s, --server: The server name
+ -u, --username: Your user name
+ -p --password: Your password
+ -n, --next-game: Adds Next Game information
+ -l, --local-info: Try to find the game details on local channels
+ -o, --output: File to save to (default nfl-epg.xml)

Output is saved in the current directory.\
\
The --next-game option will add EPG entries with Next Game: Team info\
before the games scheduled time. If you do not use this option\
then there will only be a EPG entry for when the game is scheduled.\
\
The --local-info option will try to find the game description from\
local channels instead of the generic entry found on the sports channels.\
This may include information like players and standings whereas the\
generic description will only contain team names and stadium.\
This option may take some time to run as the local EPG file is large.

# Running

cargo run -- options\
Example: cargo run -- -s SERVER -u USER -p PASSWORD

The first time you run you will notice a lot of packages being downloaded and compiled.
This is normal.

# Building
If you want to run this from a cron job, etc you need to build the project. To do this just run

cargo build --release

And you will find nfl-epg in target/release

Enjoy! And feedback is welcome!
