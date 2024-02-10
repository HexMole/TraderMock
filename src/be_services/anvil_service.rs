use lazy_static::lazy_static; // 1.4.0
use cfg_if::cfg_if;
use serde::Serialize;
use serde::Deserialize;


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractModel
{
    pub Type:String,
    pub ContractAddress: String,
    pub Name:String,
}

cfg_if! { if #[cfg(feature = "ssr")] {

    use std::{ops::Deref, sync::Mutex};

    
    use ethers::{
        contract::{abigen, ContractFactory},
        core::utils::Anvil,
        core::types::{Address},
        middleware::SignerMiddleware,
        providers::{Http, Provider, StreamExt, Ws},
        signers::{LocalWallet, Signer},
        solc::{Artifact, Project, ProjectPathsConfig},
        core::utils::{parse_ether},
    };
    use eyre::Result;
    use std::{path::PathBuf, sync::Arc, time::Duration, ptr::addr_of};

    abigen!(
        IUniswapV3Pool, "src/abi/Pool.json",
    
    );

    
    abigen!(
        UniswapV3Quoter,
        r#"[
            struct QuoteSingleParams {address tokenIn;address tokenOut;uint24 fee;uint256 amountIn;uint160 sqrtPriceLimitX96;}
            function quoteSingle(QuoteSingleParams memory params) public returns (uint256 amountOut,uint160 sqrtPriceX96After,int24 tickAfter)
            function factory() return (address)
            function getPool(address token0,address token1,uint24 fee) internal view returns (address pool)
            function test() external view returns ( uint256 amountOut)
        ]"#
    
    );
    
    abigen!(
        UniswapV3Factory,
        r#"[
            function createPool(address tokenX,address tokenY,uint24 fee) public returns (address pool)
            event PoolCreated(address indexed token0,address indexed token1,uint24 indexed fee,address pool)
        ]"#
    
    );
    
    abigen!(
        UniswapManager,
        r#"[
            struct MintParams {address tokenA;address tokenB;uint24 fee;int24 lowerTick;int24 upperTick;uint256 amount0Desired;uint256 amount1Desired;uint256 amount0Min;uint256 amount1Min;}
            struct SwapSingleParams {address tokenIn;address tokenOut;uint24 fee;uint256 amountIn;uint160 sqrtPriceLimitX96;}
            function swapSingle(SwapSingleParams calldata params) public returns (uint256 amountOut)
            function mint(MintParams calldata params) public returns (uint256 amount0, uint256 amount1)
        ]"#
    
    );
    
    abigen!(
        IERC20Contract,
        r#"[
            function mint(address to, uint256 amount)
            function balanceOf(address _owner) external view returns (uint256)
            function approve(address spender, uint256 amount) returns (bool) 
            function name() returns (string)
            event Transfer(address indexed from, address indexed to, uint256 value)
            event Approval(address indexed owner, address indexed spender, uint256 value)
        ]"#
    );
    

    lazy_static! {
        pub static ref AnvilInst: Mutex<ethers_core::utils::AnvilInstance> = Mutex::new(Anvil::new().args(["--code-size-limit", "100000"]).spawn());
        pub static ref ContractList: Mutex<Vec<ContractModel>> = Mutex::new(Vec::new());
    }

    pub fn  create_new_wallet() -> Result<Address>{
        let wallet: LocalWallet = AnvilInst.lock().unwrap().keys()[0].clone().into();
        let wallet_address:Address = wallet.address();

        return Ok(wallet_address);
    }

    pub fn add_created_contract_address(contract_name:String, address:Address,contract_type:String) 
    {
        let contractModel1=ContractModel{
            Type:contract_type.to_string(),
            ContractAddress: address.to_string(),
            Name:contract_name.clone(),
    
        };
        ContractList.lock().unwrap().push(contractModel1);
    }


    pub async fn deploy_erc20Mintable_deposit10_Weth()  { 
        // 1. compile the contract (note this requires that you are inside the `examples` directory) and
        // launch anvil

        // the directory we use is root-dir/examples
        let src_file = std::env::var("SRC_FILE").expect("SRC_FILE must be set.");
        let lib_file = std::env::var("LIB_TOML_FILE").expect("LIB_TOML_FILE must be set.");
        let root = PathBuf::from(src_file.clone()).join("contracts");
        let lib = PathBuf::from(lib_file.clone());
        // we use `root` for both the project root and for where to search for contracts since
        // everything is in the same directory
        let paths = ProjectPathsConfig::builder().root(&root).sources(&root).lib(&lib).build().unwrap();


        // get the solc project instance using the paths above
        let project = Project::builder().paths(paths).ephemeral().no_artifacts().build().unwrap();
        // compile the project and get the artifacts
        let output = project.compile().unwrap();
        let contract = output.find_first("ERC20Mintable").expect("could not find contract");


        let wallet: LocalWallet = AnvilInst.lock().unwrap().keys()[0].clone().into();
        let wallet_address:Address= wallet.address();
            // 3. connect to the network
        // 3. connect to the network
        let ws_endpoint=AnvilInst.lock().unwrap().endpoint();
        println!("ws_endpoint {}",ws_endpoint);

        let provider =
        Provider::<Ws>::connect(AnvilInst.lock().unwrap().ws_endpoint()).await.unwrap().interval(Duration::from_millis(1u64));

        let client: Arc<SignerMiddleware<Provider<Ws>, ethers_signers::Wallet<ecdsa::SigningKey<ethers_core::k256::Secp256k1>>>> = Arc::new(SignerMiddleware::new(provider, wallet.with_chain_id(AnvilInst.lock().unwrap().chain_id())));


        //let client: Arc<Arc<SignerMiddleware<Provider<Ws>, ethers_signers::Wallet<ecdsa::SigningKey<ethers_core::k256::Secp256k1>>>>> = Arc::new(client);

        let weth_contract_address = create_token_deposit( &client, contract,wallet_address,18 as u8,1,"Wrapped Ether".to_string(),"WETH".to_string()).await;
        println!("WETH : {}", weth_contract_address.0);
        add_created_contract_address("Wrapped Ether".to_string(),weth_contract_address.1,"ERC20".to_string());

        let usdc_contract_address = create_token_deposit( &client, contract,wallet_address,18 as u8,5000 as u32,"USD Coin".to_string(),"USDC".to_string()).await;
        add_created_contract_address("USD Coin".to_string(),usdc_contract_address.1,"ERC20".to_string());
        println!("USDC : {}", usdc_contract_address.0);

        let contract = output.find_first("UniswapV3Factory").expect("could not find contract");
            
        let factoryAddress: Address = create_factory(&client, contract,wallet_address).await;

        let uniswap_factory = UniswapV3Factory::new(factoryAddress, client.clone());
        add_created_contract_address("Factory Address".to_string(),factoryAddress,"UniswapV3Factory".to_string());
        let uniswap_pool: Option<ethers_core::types::TransactionReceipt> = uniswap_factory.create_pool(weth_contract_address.1,  usdc_contract_address.1, 3000).send().await.ok().unwrap().await.ok().unwrap();

        let events = uniswap_factory.event::<PoolCreatedFilter>().from_block(0);
        let mut stream = events.stream().await.unwrap().take(1);
        let mut pool_address=None;
        while let Some(Ok(f)) = stream.next().await {
            println!("MintFilter event: {f:?}");
            pool_address=Some(f.pool);
        }
        if let Some(add) = pool_address {
            add_created_contract_address("Pool USDC-ETH".to_string(),add,"UniswapV3Pool".to_string());
            let pool_contract = IUniswapV3Pool::new(add, client.clone());
            let token0= pool_contract.token_0().call().await.unwrap();
            let token1= pool_contract.token_1().call().await.unwrap();
            println!("token0: {}", token0);
            println!("token1: {}", token1);

            let contract = output.find_first("UniswapV3Manager").expect("could not find contract");

            let manager_address = create_manager(&client,contract,factoryAddress).await;

            let uniswap_manager = UniswapManager::new(manager_address, client.clone());
            add_created_contract_address("UniswapManager".to_string(),manager_address,"UniswapV3Manager".to_string());

            let rsl_approved1 =approve_erc20(&client,token0,manager_address,1).await;
            let rsl_approved2 =approve_erc20(&client,token1,manager_address,5000).await;

            let min_params = MintParams{
                token_a:token0,
                token_b:token1,
                fee:3000,
                amount_0_min:parse_ether(0).unwrap(),
                amount_1_min:parse_ether(0).unwrap(),
                amount_0_desired:parse_ether(1).unwrap(),
                amount_1_desired:parse_ether(5000).unwrap(), 
                lower_tick:84240,
                upper_tick:86100
            };
            uniswap_manager.mint(min_params).send().await.unwrap().await.unwrap();
            println!("uniswap pool minted!");

            let events = pool_contract.events().from_block(0);
            let mut stream = events.stream().await.unwrap().take(1);


            // let mut pool_address=None;
            println!("stream...");
            while let Some(Ok(f)) = stream.next().await {
                println!("MintFilter event: {f:?}");
                // pool_address=Some(f.pool);
            }

            
        }

    }

    async fn create_manager(
        client:&Arc<SignerMiddleware<Provider<Ws>, ethers_signers::Wallet<ecdsa::SigningKey<ethers_core::k256::Secp256k1>>>>,
        contract:&ethers::solc::ConfigurableContractArtifact,
        factory_address:Address) -> Address
    {
            let (abi, bytecode, _) = contract.clone().into_parts();
            // 5. create a factory which will be used to deploy instances of the contract
            let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());

            // 6. deploy it with the constructor arguments

            let manager_address = factory.deploy(factory_address);

            let manager_contract =manager_address.unwrap().send().await.unwrap();

            println!("factory_address: {}", factory_address);
            
            let manager_contract_address = manager_contract.address();

            return manager_contract_address;
    }

    async fn approve_erc20(
        client:&Arc<SignerMiddleware<Provider<Ws>, ethers_signers::Wallet<ecdsa::SigningKey<ethers_core::k256::Secp256k1>>>>,
        erc20_address:Address,
        spender_address:Address,
        ammout: u64)
    {
        let weth_erc20_contract = IERC20Contract::new(erc20_address, client.clone());
        weth_erc20_contract.approve(spender_address, parse_ether(ammout).unwrap()).send().await.unwrap().await.unwrap();
    }

    async fn create_factory(
        client:&Arc<SignerMiddleware<Provider<Ws>, ethers_signers::Wallet<ecdsa::SigningKey<ethers_core::k256::Secp256k1>>>>,
        contract:&ethers::solc::ConfigurableContractArtifact,
        wallet_Address:Address) -> Address
    {
            let (abi, bytecode, _) = contract.clone().into_parts();
            // 5. create a factory which will be used to deploy instances of the contract
            let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());

            // 6. deploy it with the constructor arguments

            let weth_Erc20_mint = factory.deploy(());

            let weth_erc20 =weth_Erc20_mint.unwrap().send().await.unwrap();

            println!("wallet_Address: {}", wallet_Address);
            let factory_contract_address = weth_erc20.address();

            return factory_contract_address;
    }

    async fn create_token_deposit(
        client:&Arc<SignerMiddleware<Provider<Ws>, ethers_signers::Wallet<ecdsa::SigningKey<ethers_core::k256::Secp256k1>>>>,
        contract:&ethers::solc::ConfigurableContractArtifact,
        wallet_Address:Address,
        number_of_decimal_places:u8,
        mint_token_amount:u32,
        name:String,
        symbol:String) -> (ethers_core::types::U256,Address)
    {
            let (abi, bytecode, _) = contract.clone().into_parts();
            // 5. create a factory which will be used to deploy instances of the contract
            let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());

            // 6. deploy it with the constructor arguments

            let params=(name, symbol, 18 as u8);
            let weth_Erc20_mint = factory.deploy(params);

            let weth_erc20 =weth_Erc20_mint.unwrap().send().await.unwrap();

            println!("wallet_Address: {}", wallet_Address);
            let erc20_address = weth_erc20.address();

            // 8. instantiate the contract
            let weth_erc20_contract = IERC20Contract::new(erc20_address, client.clone());


            let rsl = weth_erc20_contract.mint(wallet_Address, parse_ether(mint_token_amount).unwrap()).send().await.ok().unwrap().await.ok().unwrap();
            // println!("rsl: {}", serde_json::to_string(&rsl).ok().unwrap());

            let balanceOfTheContractwethErc20: ethers_core::types::U256 = weth_erc20_contract.balance_of(wallet_Address).call().await.ok().unwrap();
        
            println!("balanceOfTheContractwethErc20 : {}", balanceOfTheContractwethErc20);
            return (balanceOfTheContractwethErc20,erc20_address);
    }

}}