// A summary explaining the purpose of this contract (should fit in 5-6 lines)
/**
 *  One Starknet SimpleVault contact which accept user deposit token(binding the toke during deployed), then mints its own ERC2O calculated share  token to user.
 *  1. First define the interface IERC20<TContractState>, which it's the default functions for the SimpleVault contract???
 *  2. Deine the interface ISimpleVault<TContractState>, then SimpleVault implement them as public funcitons. which as the core logic.
 *  3. deposit: user deposit token amount, SimpleVault mint the share amount to the user meanwhile updating total_supply and balance_of
 *  4. withdraw: user withdraw the expect token amount based on the input  SimpleVault share token amount meanwhile updating total_supply and balance_of.
 * 
 */


//  The `ContractAddress` type represents a Starknet contract address, with a value range of  `[0, 2**251)`.
use starknet::ContractAddress;

// Make below public traits's funcitons as the interface, which is public.
#[starknet::interface]
// type TContractState that represents the current contract state. 
pub trait IERC20<TContractState> {
    // All below funciton are the standard ETHEREUM ERC20 interface function
    /**
     * The differences:
     *  1. use Cario data type felt252
     *  2. if the function don't change the contract state, input self: @TContractState. 
     *       othereise ref self: TContractState which will modity the contract state. 
     *        Can see: transfer/transfer_from/approve/increase_allowance/decrease_allowance
     *  3. Execept the first input param is the contract state iteself, 
     *      other params use the type ContractAddress for other contract or user, or felt252 for the amount
     */
    fn get_name(self: @TContractState) -> felt252;
    fn get_symbol(self: @TContractState) -> felt252;
    fn get_decimals(self: @TContractState) -> u8;
    fn get_total_supply(self: @TContractState) -> felt252;
    fn balance_of(self: @TContractState, account: ContractAddress) -> felt252;
    fn allowance(
        self: @TContractState, owner: ContractAddress, spender: ContractAddress
    ) -> felt252;
    fn transfer(ref self: TContractState, recipient: ContractAddress, amount: felt252);
    fn transfer_from(
        ref self: TContractState,
        sender: ContractAddress,
        recipient: ContractAddress,
        amount: felt252
    );
    fn approve(ref self: TContractState, spender: ContractAddress, amount: felt252);
    fn increase_allowance(ref self: TContractState, spender: ContractAddress, added_value: felt252);
    fn decrease_allowance(
        ref self: TContractState, spender: ContractAddress, subtracted_value: felt252
    );
}


// Same pattern for pub trait IERC20<TContractState>, but define a separate trait ISimpleVault which aim for token transfer and accounting
#[starknet::interface]
pub trait ISimpleVault<TContractState> {
    fn deposit(ref self: TContractState, amount: u256);
    fn withdraw(ref self: TContractState, shares: u256);
    fn user_balance_of(ref self: TContractState, account: ContractAddress) -> u256;
    fn contract_total_supply(ref self: TContractState) -> u256;
}

// Define SimpleVault as a contract
#[starknet::contract]
pub mod SimpleVault {
    //  Below supply the gateWay calling other contract's functions. like IERC20(instance).methods. 
    //  IERC20Dispatcher:stuct? IERC20DispatcherTrait this generated the related functions by compiler.
    use super::{IERC20Dispatcher, IERC20DispatcherTrait};
    //  ContractAddress Type represent the caller 
    //  get_caller_address,get_contract_address functions as the literal meaning shows. like msg,sender, address(this)
    use starknet::{ContractAddress, get_caller_address, get_contract_address};

    // As the annotation shows, the storage struct store the on-chain related data 
    #[storage]
    struct Storage {
        // token type as IERC20Dispatcher. like IERC20(token)
        token: IERC20Dispatcher,
        // Accounting how many token were minted?
        total_supply: u256,
        // LegacyMap like hashmap, accounting each user's minted token
        balance_of: LegacyMap<ContractAddress, u256>
    }

    // As the annotation shows, below funciton as constructor. can only be called when deploy the smart contract.
    #[constructor]
    // ref self: ContractState. contract iteself. ref means can change the contract states.
    // token: ContractAddress: related with IERC20Dispatcher. An ERC20 token address.
    fn constructor(ref self: ContractState, token: ContractAddress) {
        // self.token ==> struct Storage.token
        // binding the self.takne as the inpyt token by using the write function.
        self.token.write(IERC20Dispatcher { contract_address: token });
    }

    // tells the compiler to generate the corresponding trait definition
    #[generate_trait]
    // the traits functions is private functions. like private functions in Ethereum
    impl PrivateFunctions of PrivateFunctionsTrait {
        // mint share amount to toContractAddress meanwhile updating the total_supply and each user's balance
        //  also the first param is ref self contract itself which means can change.
        fn _mint(ref self: ContractState, to: ContractAddress, shares: u256) {
            // update struct Storage.total_supply by applying write function
            // Formula: read the struct Storage.total_supply then add input shares to it. 
            self.total_supply.write(self.total_supply.read() + shares);
            // update struct Storage.balance_of by applying write function
            // Formula: read the struct Storage.balance_of(to) get to's current share then add input shares, finally updating toAddress's balance.
            self.balance_of.write(to, self.balance_of.read(to) + shares);
        }
        // burn share amount for the from address. dreasing the total_supply and related user's balance.
        fn _burn(ref self: ContractState, from: ContractAddress, shares: u256) {
            self.total_supply.write(self.total_supply.read() - shares);
            self.balance_of.write(from, self.balance_of.read(from) - shares);
        }
        
    }

    // According to the doc introduction, #[abi(embed_v0)] means public functions.
    #[abi(embed_v0)]
    // As above define ISimpleVault<TContractState>, SimpleVault implement all four functions.
    impl SimpleVault of super::ISimpleVault<ContractState> {

        // query account balance. 
        // ?? no need ref?
        fn user_balance_of(ref self: ContractState, account: ContractAddress) -> u256 {
            self.balance_of.read(account)
        }   
        // query all totol Supply.
        // ?? no need ref?
        fn contract_total_supply(ref self: ContractState) -> u256 {
            self.total_supply.read()
        }

        // The caller deposit amount for  struct Storage.token 
        fn deposit(ref self: ContractState, amount: u256){
            // get caller's address
            let caller = get_caller_address();
            // contract iteself address
            let this = get_contract_address();

            // Calculate the share: 
            // first time share == amount
            // Formula for  Non first time: share =  (amount * current token total supply) / current contract's token balance 
            let mut shares = 0;
            if self.total_supply.read() == 0 {
                shares = amount;
            } else {
                let balance: u256 = self.token.read().balance_of(this).try_into()
                .unwrap();
                shares = (amount * self.total_supply.read()) / balance;
            }
           // Call this contract's private _mint function. which mint the share to caller and do the accounting task
           PrivateFunctions::_mint(ref self, caller, shares);
            
            // amount type conversion, cairo own methond
            // low??
            let amount_felt252: felt252 = amount.low.into();
            // token.read() return the token instance than call its transfer_from function. (token: IERC20Dispatcher)
            // Transfer amount_felt252 token from caller to this contract
            self.token.read().transfer_from(caller, this, amount_felt252);
        }

        // caller withdraw share amount token 
        fn withdraw(ref self: ContractState, shares: u256) {
            let caller = get_caller_address();
            let this = get_contract_address();

            // balance: the contract iteself balance for its token
            // ?? seems below always equal zero. should self.user_balance_of(caller)?? user transfer SimpleVault token to this contract?
            let balance = self.user_balance_of(this);
            // Formula:  (shares *  the contract iteself balance for its token) / total_supply
            let amount = (shares * balance) / self.total_supply.read();
            // Call private function _burn
            PrivateFunctions::_burn(ref self, caller, shares);
            let amount_felt252: felt252 = amount.low.into();
            // Tranfer amount_felt252 token to caller
            self.token.read().transfer(caller, amount_felt252);
        }
    }
}