pub fn ethers_boilerplate() -> &'static str {
    r#"
    import { ethers } from "ethers";
    import { errors } from "./jasper-bindings";
    
    type DebugArg = {
      value: string;
      name: string;
    };
    export const jasperParseError = (errorData: string) => {
      const first4Bytes = errorData.slice(0, 10);
      const error = errors[first4Bytes];
      if (!error) {
        throw new Error(`Unknown error: ${errorData}`);
      }
    
      const abiCoder = new ethers.utils.AbiCoder();
    
      let errorMessage = error.solidityMessageAndArgs.errorMessage;
    
      const _errorData = `0x${errorData.slice(10)}`;
    
      let debugArgs: DebugArg[] = [];
      if (error.solidityMessageAndArgs.args.length > 0) {
        const decoded = abiCoder.decode(
          error.solidityMessageAndArgs.args.map((arg) => arg.type),
          _errorData
        );
        error.solidityMessageAndArgs.args.forEach((arg, i) => {
          const value =
            typeof decoded[i] === "string" ? decoded[i] : decoded[i].toString();
          errorMessage = errorMessage.replace(`{${arg.name}}`, value);
          debugArgs.push({ name: arg.name, value });
        });
      }
      return {
        error,
        errorMessage,
        debugArgs,
      };
    };
    "#
}

pub fn viem_boilerplate() -> &'static str {
    r#"
    import { decodeAbiParameters } from "viem";
    import { errors } from "./jasper-bindings";
    
    type DebugArg = {
      value: string;
      name: string;
    };
    
    export const jasperParseError = (errorData: string) => {
      const first4Bytes = errorData.slice(0, 10);
      const error = errors[first4Bytes];
      if (!error) {
        throw new Error(`Unknown error: ${errorData}`);
      }
    
      let errorMessage = error.solidityMessageAndArgs.errorMessage;
    
      const _errorData = ("0x" + errorData.slice(10)) as `0x${string}`;
      let debugArgs: DebugArg[] = [];
      if (error.solidityMessageAndArgs.args.length > 0) {
        const namesAndValues = decodeAbiParameters(
          error.solidityMessageAndArgs.args,
          _errorData
        );
    
        //Replace all {name} with {namesAndValues[index of name]}
        for (let i = 0; i < error.solidityMessageAndArgs.args.length; i++) {
          let value = namesAndValues[i];
          if (typeof value !== "string") {
            //@ts-ignore
            value = value.toString();
          }
          errorMessage = errorMessage.replace(
            `{${error.solidityMessageAndArgs.args[i].name}}`,
            `${value}`
          );
          debugArgs.push({
            name: error.solidityMessageAndArgs.args[i].name,
            value: `${value}`,
          });
        }
      }
      return {
        error,
        errorMessage,
        debugArgs,
      };
    }; 
    "#
}
