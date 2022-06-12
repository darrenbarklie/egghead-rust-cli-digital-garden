# Digital Garden

A CLI tool for the creation and maintainence of Digital Gardens.

## Developer

### Run

```shell
cargo watch -x check -x test
```

### Setting the garden path

```shell
GARDEN_PATH=~/DigitalGarden/ write
garden -p ~/DigitalGarden/ write
garden --garden_path ~/DigitalGarden write
```

## Commands

### write

Open a new file to write in our digitial garden. Since we don't necessarily know what we want to title what we are wrting, we will leave the title as optional and ask the user to it later if they don't provide it up front.

```shell
garden write
garden write -t "Some Title"
```
