<img src="assets/jesper.webp" alt="Jesper" style="border-radius: 10px; width: 200px; height: 200px;"/>

# Jesper

Jesper is a simple, yet powerful, error generator for Solidity. It allows you to generate typescript errors from your Solidity code, and use them to create custom error messages.

## Installation

```bash
npm install jasper
```

## Usage

Jesper automatically parses the output folder of your framework and generates a typescript file with all the errors in it. You can then import this file and use it to create custom error messages.

You can also optionally leave a message on top of the error which will add a custom message to the error.
The error arguments can also be injected into the message by using the `{}` syntax.

Rules

- The error message must be on the same line as the error declaration
- The error message must start with `//#Message:`
- The error message must be on the same line as the error declaration
- The injected arguments must be wrapped in `{}`
- The error must be wrapped in quotes ""

```typescript
contract MyContract {
    error ErrIsContract();
    //#Message: "USDG: NotVetoCouncilMember"
    error ErrNotVetoCouncilMember();
    //#Message: "USDG: PermanentlyFrozen"
    error ErrPermanentlyFrozen();
    //#Message: "USDG: ToCannotBeUSDCReceiver"
    error ToCannotBeUSDCReceiver();
    //#Message: "USDG: CannotSwapZero"
    error ErrCannotSwapZero();

    //#Message: "Cannot send to {user}"
    error ErrCannotSendToUser(address user, address sender);

   //......rest of logic

}
```

## Commands

### Init project

This creates a default `jesper-config.json` file in the root of your project.

`jesper init`

### Generate errors

`jesper gen`

## Config

```typescript
{
  "outputFolder": "./jesper-bindings", // The folder where the generated typescript file will be placed
  "typescript": true, //does nothing for now, will get JS bindings soon
  "framework": "foundry", //foundry | hardhat
  "contractsPath": "./contracts", //The path where the contracts are located
  "excludedFiles": ["./contracts/Migrations.sol"], //Files to exclude, doesen't work yet
  "extraIncludedFiles": [], //Extra files to include, doesen't work yet
  "modes": ["Viem"] //Viem | EthersV5
}

```

## About The Output

Jesper will output a typescript file with all the errors in it. The file will be named `jesper-bindings.ts` and will be placed in the `outputFolder` specified in the config.

Another file will be generated based on the `modes` specified in the config. This file will be named `jesperParseError<mode>.ts` and will be placed in the `outputFolder` specified in the config as well.

It exports a function called `jesperParseError` which takes in error data and returns a string. This function is used to parse the error message and inject the arguments into the message.

## Giving Feedback

I'm not a rust expert nor a crate versioning expert. Forgive any silly errors, I just needed a quick solution to reduce the lag between frontend, backend, and contracts. If you want to change something feel free to open a PR or an issue.

## Roadmap

- [x] Make sure you can't inject an error argument that doesen't exist.
- [x] Setup tests
- [x] Setup CI
- [x] More stuff I can't think of, just did a 12h bender on this

## Author

Made by <a href="https://x.com/0xSimon" target="_blank">0xSimon</a>
