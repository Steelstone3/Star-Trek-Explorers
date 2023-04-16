# Star Trek Explorers

A test driven Rust version based on the original 1970s console game in the genre of "rogue-like" and is proceedurally generated using ASCII graphics.

## Running Star Trek Explorers

> cd ~/StarTrekExplorers/StarTrekExplorers
>
> dotnet restore
>
> dotnet build
>
> dotnet run

This application has been tested to run on debain derived Linux, Windows 10 and Mac OS 10 beyond this scope your experiences may vary.

### Tests

> cd ~/StarTrekExplorers/StarTrekExplorersTests
>
> dotnet restore
>
> dotnet build
>
> dotnet test

### Dependencies

Follow the steps for installing dotnet runtime for your given operating system.

> <https://dotnet.microsoft.com/en-us/download/dotnet/7.0>

Install the following .net tool and use its upgrade feature to keep 3rd party packages updated

> dotnet tool install --global dotnet-outdated-tool
>
> dotnet outdated --upgrade
