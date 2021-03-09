# UDP Even Odd Game
A simple even odd game using over a upd connection.

## Build Requirements

- [Rust 1.50+](https://www.rust-lang.org/tools/install)

## How to build

Clone the repository

```shell script
git clone 
```

Enter the repository folder

```shell script
cd repository_folder
```

Build the project

```shell script
cargo build 
```

To make a release build just use the `--release` flag:

```shell script
cargo build --release 
```

The binary file can be found at the folder `target/release`, for release build, or `target/debug` for the debug build. The name of the binary is __even_odd_game__.

## How to Play

To play the game you have to build it in you machine first. 

### Start the server and the clients
First start the server

```shell script
./even_odd_game server 
```

__the port used for the serve the game is 34254, on your local machine__

Start a client, in another machine/terminal:

```shell script
./even_odd_game client
```

To specify the server address you can use the third argument, e.g.:

```shell script
./even_odd_game client 192.168.0.2:34254
```

Then, start another client in another machine/terminal. Remember to specify the server address in case you open in another machine.

### Playing the game
When asked to input the value, you've to pass your play in the following way:

```shell script
OPTION_NUMBER VALUE 
```

e.g. To play Even with 4:

```shell script
1 4 
```

Here is the list of available plays:

|Option|Option Number|Value|
| :------------- | :----------: | -----------: |
|Even|1|\[1-5\]|
|Odd|2|\[1-5\]|