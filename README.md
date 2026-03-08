# Monod

## Disclaimer

Monod is still in early development so it is practically unusable and many of the announced features have yet to be implemented. Feel free to participate in the project by checking out the <a href="#contributing">Contributing</a> section.

## About The Project

Monod is a system monitor similar to [btop](https://github.com/aristocratos/btop), however this one is written in rust and highly customizable. From the theme and the layout to the choice of the characters, everything can be adjusted to your liking.

## Getting Started

### Prerequisites

To install Monod you need Rust installed.

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Manual Installation

To manually install Monod you first need to download this repository.

```sh
git clone https://github.com/Ribzee/monod.git
cd monod
```

Then you can build the project.

```sh
cargo build --release
cd target/release
```

Done! You can use it with

```sh
./monod
```

## Usage

Simply run

```sh
monod
```

Alternatively you can find the command line options with:

```sh
monod -h
```

## Roadmap

- [ ] Add processes and hardware usage
- [ ] Add basic terminal interface
- [ ] Add user input
- [ ] Add TUI customization
  - [ ] Add config file parsing
  - [ ] Add customizable widgets
  - [ ] Add customizable layout
  - [ ] Add themes

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Top contributors

<a href="https://github.com/Ribzee/monod/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=Ribzee/monod" alt="contrib.rocks image" />
</a>

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.

<!-- CONTACT -->
## Contact

Ribzee - [ribbzee](https://discord.com/users/856572381023174657) - <cece06pesle@gmail.com>

Project Link: [https://github.com/Ribzee/monod](https://github.com/Ribzee/monod)
