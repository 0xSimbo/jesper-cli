use serde::{Deserialize, Serialize};
use std::collections::HashMap;
mod find_selector;
mod parse_sol_file;
use parse_sol_file::{parse_sol_file, SolidityMessageAndArgs};
mod constants;
mod prepare_error_from_abi;
use clap::{arg, command, value_parser, ArgAction, Command, Parser};
pub mod handle_parse_all_files;
mod read_config;
use read_config::read_config;
use crate::handle_parse_all_files::handle_parse_all_files;
#[derive(Parser, Deserialize, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    arg: String,
}

fn main() {
    //read in abi.json and parse it
    // call the cli jasper
    
    let config = read_config();
    let all_abi_files = config.all_abi_files.clone();
    let all_solidity_files = config.all_solidity_files.clone();
    handle_parse_all_files(all_abi_files, all_solidity_files,&config);
    // let matches = Command::new("jasper")
    //     .version("1.0")
    //     .author("Your Name <your_email@example.com>")
    //     .about("Description of what jasper does")
    //     .get_matches();

    // let args = Args::parse();
    // println!("args = {:?}", args);
    
    // let contracts: Vec<String> = vec![
    //     "contracts/CarbonCreditDescendingPriceAuction.sol".to_string(),
    //     "contracts/ImpactCatalyst.sol".to_string(),
    //     "contracts/GCC.sol".to_string(),
    //     "contracts/GLOW.sol".to_string(),
    //     "contracts/USDG.sol".to_string(),
    //     "contracts/GrantsTreasury.sol".to_string(),
    //     "contracts/Governance.sol".to_string(),
    //     "contracts/SafetyDelay.sol".to_string(),
    //     "contracts/EarlyLiquidity.sol".to_string(),
    //     "contracts/BatchCommit.sol".to_string(),
    //     "contracts/GlowUnlocker.sol".to_string(),
    //     "contracts/MinerPoolAndGCA/BucketSubmission.sol".to_string(),
    //     "contracts/MinerPoolAndGCA/GCA.sol".to_string(),
    //     "contracts/MinerPoolAndGCA/GCASalaryHelper.sol".to_string(),
    //     "contracts/MinerPoolAndGCA/MinerPoolAndGCA.sol".to_string(),
    //     "contracts/MinerPoolAndGCA/mock/MockSalaryHelper.sol".to_string(),
    //     "contracts/MinerPoolAndGCA/mock/MockMinerPoolAndGCA.sol".to_string(),
    //     "contracts/MinerPoolAndGCA/mock/MockGCA.sol".to_string(),
    //     "contracts/VetoCouncil/VetoCouncilSalaryHelper.sol".to_string(),
    //     "contracts/VetoCouncil/VetoCouncil.sol".to_string(),
    //     "contracts/GuardedLaunch/Glow.GuardedLaunch.sol".to_string(),
    //     "contracts/GuardedLaunch/GCC.GuardedLaunch.sol".to_string(),
    //     "contracts/temp/MatrixPayout.sol".to_string(),
    //     "contracts/temp/GCAPayoutAlgo.sol".to_string(),
    //     "contracts/temp/MinerArray.sol".to_string(),
    //     "contracts/temp/Math.sol".to_string(),
    //     "contracts/temp/TestBucketMath.sol".to_string(),
    //     "contracts/temp/GCAPayoutAlgo2.sol".to_string(),
    //     "contracts/temp/MD2.sol".to_string(),
    //     "contracts/temp/MinerDistributionMath.sol".to_string(),
    //     "contracts/UniswapV2/contracts/test/WETH9.sol".to_string(),
    //     "contracts/Constants/Constants.sol".to_string(),
    //     "contracts/libraries/ABDKMath64x64.sol".to_string(),
    //     "contracts/libraries/UniswapV2Library.sol".to_string(),
    //     "contracts/libraries/VestingMathLib.sol".to_string(),
    //     "contracts/libraries/HalfLifeCarbonCreditAuction.sol".to_string(),
    //     "contracts/libraries/HalfLife.sol".to_string(),
    //     "contracts/testing/ERC20.sol".to_string(),
    //     "contracts/testing/MockUSDCTax.sol".to_string(),
    //     "contracts/testing/MockUSDC.sol".to_string(),
    //     "contracts/testing/MockGovernance.sol".to_string(),
    //     "contracts/testing/TestGLOW.sol".to_string(),
    //     "contracts/testing/TestUSDG.sol".to_string(),
    //     "contracts/testing/TestGCC.sol".to_string(),
    //     "contracts/testing/GuardedLaunch/TestGCC.GuardedLaunch.sol".to_string(),
    //     "contracts/testing/GuardedLaunch/GoerliGLOW.GuardedLaunch.sol".to_string(),
    //     "contracts/testing/GuardedLaunch/TestGLOW.GuardedLaunch.sol".to_string(),
    //     "contracts/testing/GuardedLaunch/GoerliGCC.GuardedLaunch.sol".to_string(),
    //     "contracts/testing/EarlyLiquidity/EarlyLiquidityMockMinerPool.sol".to_string(),
    //     "contracts/testing/Goerli/GoerliMinerPoolAndGCA.QuickPeriod.sol".to_string(),
    //     "contracts/testing/Goerli/Constants.QuickPeriod.sol".to_string(),
    //     "contracts/testing/Goerli/GoerliGovernance.QuickPeriod.sol".to_string(),
    //     "contracts/testing/Goerli/GoerliGCC.sol".to_string(),
    //     "contracts/UnifapV2/UnifapV2Pair.sol".to_string(),
    //     "contracts/UnifapV2/UnifapV2Factory.sol".to_string(),
    //     "contracts/UnifapV2/UnifapV2Router.sol".to_string(),
    //     "contracts/UnifapV2/libraries/UQ112x112.sol".to_string(),
    //     "contracts/UnifapV2/libraries/UnifapV2Library.sol".to_string(),
    //     "contracts/UnifapV2/libraries/Math.sol".to_string(),
    //     "contracts/UnifapV2/interfaces/IUnifapV2Factory.sol".to_string(),
    //     "contracts/UnifapV2/interfaces/IERC20.sol".to_string(),
    //     "contracts/UnifapV2/interfaces/IUnifapV2Pair.sol".to_string(),
    //     "contracts/interfaces/IVetoCouncil.sol".to_string(),
    //     "contracts/interfaces/IERC20Permit.sol".to_string(),
    //     "contracts/interfaces/IGrantsTreasury.sol".to_string(),
    //     "contracts/interfaces/IUniswapRouterV2.sol".to_string(),
    //     "contracts/interfaces/IMinerPool.sol".to_string(),
    //     "contracts/interfaces/IGlow.sol".to_string(),
    //     "contracts/interfaces/IGovernance.sol".to_string(),
    //     "contracts/interfaces/IEarlyLiquidity.sol".to_string(),
    //     "contracts/interfaces/ICarbonCreditAuction.sol".to_string(),
    //     "contracts/interfaces/IUniswapV2Pair.sol".to_string(),
    //     "contracts/interfaces/IGCC.sol".to_string(),
    //     "contracts/interfaces/IGCA.sol".to_string(),
    // ];

    // let all_abis = vec![
    //     "abis/SafetyDelay.json".to_string(),
    //     "abis/CarbonCreditDescendingPriceAuction.json".to_string(),
    //     "abis/GrantsTreasury.json".to_string(),
    //     "abis/GCCGuardedLaunch.json".to_string(),
    //     "abis/MinerPoolAndGCA.json".to_string(),
    //     "abis/EarlyLiquidity.json".to_string(),
    //     "abis/Governance.json".to_string(),
    //     "abis/USDG.json".to_string(),
    //     "abis/GlowGuardedLaunch.json".to_string(),
    //     "abis/VetoCouncil.json".to_string(),
    //     "abis/ImpactCatalyst.json".to_string(),
    // ];
    // handle_parse_all_files(all_abis, contracts);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
