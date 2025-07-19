/**
 * Goal: Analyze a smart contract written in Rust for the NEAR blockchain. ⚠️ This is not a security analysis.
 * Note: Some concepts have not been explained yet, give it your best!* 
 *
 * Expected outputs:
 * 
 *  - A summary explaining the purpose of this contract (should fit in 5-6 lines)
 *  
 *      Explicitly add three functions to one near smart contract.
 *      add_message: This function accept near token. When a sender deposit near token, record one PostedMessage in the Contract's messages. 
 *      each postedMessage include premium(whether or not the deposit amount greater than POINT_ONE), the sender account, the record String.
 *      get_messages: return the expected postedMessages by specify the from_index and size.
 *      total_messages: return how many postedMessages were stored in this Contract.        
 * 
 *  - An in-depth analysis of the contract. Comments should be added to the code snippet to explain the concepts shown in Lecture of Week 2.
 * 
 * 
 * 
 */

// 1. check the usage for pattern: #[serde(crate = "near_sdk::serde")]
// 2. #[cfg(test)] [test]
// 3.  some details check
// 4. from_index should check whether or not include from_index

// below import near_sdk cate, includes structs,traits,macro, near defined U64
// BorshDeserialize trait: A data-structure that can be de-serialized from binary format by NBOR.
// BorshSerialize trait: A data-structure that can be serialized into binary format by NBOR.
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
// Near wrapper U64 type
use near_sdk::json_types::U64;
// Serialize Tratit: Serialize this value into the given Serde serializer.
use near_sdk::serde::Serialize;
// Near Wrappered  Vector 
use near_sdk::store::Vector;
// env: Near Blockchain-specific methods library
// AccountId: near struct AccountId which includes many methods implemented from the related tratis
// NearToken: near struct NearToken, includes many methods as AccountId, like eth native function.
use near_sdk::{env, near_bindgen, AccountId, NearToken};

// Near token unit conversion: 100 * 10_u128.pow(21)
const POINT_ONE: NearToken = NearToken::from_millinear(100);

// implement three traits's default function:(BorshDeserialize, BorshSerialize, Serialize) for struct PostedMessage
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
// As above traits based on the near_sdk, should specify which crate in below. 
#[serde(crate = "near_sdk::serde")]
#[borsh(crate = "near_sdk::borsh")]
// define a public struct, and all files are public
pub struct PostedMessage {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

/**
 * [near_bindgen]  AI anwser:
 * The key takeaway is that the #[near_bindgen] macro is doing the heavy lifting, 
 * transforming your simple struct into a full-featured blockchain smart contract with all the necessary blockchain-specific functionality.
 * 
 */

#[near_bindgen] 
// Same as PostedMessage, implement traits(BorshDeserialize,BorshSerialize) default function for Contract
#[derive(BorshDeserialize, BorshSerialize)]
// Above traits based on the near sdk cate. should add in below.
#[borsh(crate = "near_sdk::borsh")]
// Define a public struct Contract, includes one message whose type is Vector<PostedMessage>, PostedMessage also is custome defined struct.
pub struct Contract {
    messages: Vector<PostedMessage>,
}

// Implement Default Traits for Contract, current implementation return "Vector::new(b"m")" as the default value.
impl Default for Contract {
    fn default() -> Self {
        Self {
            messages: Vector::new(b"m"),
        }
    }
}

// #[near_bindgen] seems same as #[near_bindgen] fro struct Contract. add necessar near block-chain related functionality.
#[near_bindgen]
impl Contract {
    // Mark this function accpet near token
    #[payable]
    // only accpet mutable reference Contract, which means can change the Contract iteself data.
    pub fn add_message(&mut self, text: String) {
        // if the deposited near token amount >POINT_ONE, premium as true. otherwise false
        let premium = env::attached_deposit() >= POINT_ONE;
        // by calling near block chain buitlin funciton(predecessor_account_id), get the sender account. (like ethereum msg.sender?)
        let sender = env::predecessor_account_id();
        // Create a PostedMessage struct stance in memory
        let message: PostedMessage = PostedMessage {
            premium,
            sender,
            text,
        };
        // As self.messages is Vector, the accept type is PostedMessage. add the new PostedMessage to the Vector messages.
        self.messages.push(message);
    }

    // get_messages: accept immutable reference self(contract), both the type of from_index and limit is U64, return Vector, but the type is immutable reference  PostedMessage
    pub fn get_messages(&self, from_index: Option<U64>, limit: Option<U64>) -> Vec<&PostedMessage> {
        // check the input from_index and limit, if doesn't fit the expected type: from==> 0, limit==>10
        let from = u64::from(from_index.unwrap_or(U64(0)));
        let limit = u64::from(limit.unwrap_or(U64(10)));
        
        /**
         * iter return an iterator
         * skip from_index(no including from_index). the max size is limit.
         * finally transforms an iterator into a collection.
         * 
         * 
         **/ 
        self.messages
            .iter()
            .skip(from as usize)
            .take(limit as usize)
            .collect()
    }

    // Call the Vector's default method len() return how many messages are stored.
    pub fn total_messages(&self) -> u32 {
        self.messages.len()
    }
}

// macro config the necessary configuration for uint test ?
#[cfg(test)]
// mod. Organize code 
mod tests {
    // super specify the path that which function can called? Does it means all above functions can be called by the below test functions?
    use super::*;
    
    // Attribute macro applied to a function to turn it into a unit test.
    #[test]
    fn add_message() {
        // Create a default contract
        let mut contract = Contract::default();
        // add text: "A message", the attached_deposit is equal 0. 
        contract.add_message("A message".to_string());
        // input none when calling get_messages, which will apply the default params: from_index:0, limit:10. 
        // the result return the first element in the messsages vector. 
        let posted_message = &contract.get_messages(None, None)[0];
        // as attached_deposit is equal 0. so posted_message.premium is false
        assert_eq!(posted_message.premium, false);
        assert_eq!(posted_message.text, "A message".to_string());
    }

    #[test]
    fn iters_messages() {
        // Same as above add_message, add thress messages
        let mut contract = Contract::default();
        contract.add_message("1st message".to_string());
        contract.add_message("2nd message".to_string());
        contract.add_message("3rd message".to_string());

        let total = &contract.total_messages();
        // total_messages accpet reference contract, the return len value also is reference.
        // when want to get the actual vaule for the reference, should use dereference. * ????
        assert!(*total == 3);
        // Only get messages from 2 to 4. actually return [2,3]. the fianl resut return 3th
        let last_message: &&PostedMessage = &contract.get_messages(Some(U64::from(1)), Some(U64::from(2)))[1];
        // no deposit token, the premium still as false
        assert_eq!(last_message.premium, false);
        // Based on above analyze, the 3th 
        assert_eq!(last_message.text, "3rd message".to_string());
    }
}


