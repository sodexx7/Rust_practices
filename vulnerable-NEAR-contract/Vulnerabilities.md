## 1. Bob(Attacker) can mint many times as he can, as the type converted flaw, which can leads to the existed token id belongs to Bob along with losting the expected tokeId to Bob.

The id type is u8. but the actual supply type is u16. Each time will convert the u16 to u8 when mint a token. After u8::Max times mint, the new token Id will begin from zero.

For example: 256 mint times. admin mint tokenId==0, Bob mint tokenID 1=>255 times, the follwoing mint token id will begin from 0. 256u16==>0u8

POC
[exploit_BobGetAdminToken](https://github.com/sodexx7/Rust_practices/blob/274a821bdcd388f3d2514130fa4f5887cf7612e9/vulnerable-NEAR-contract/src/lib.rs#L88)

Solution1 , change supply type.

```
--- pub supply: u16
+++ pub supply: u8
```

Solution1
change the pub type Id = u8; to u16. but should adjust below function to compitable with u16(to check)

```
self.tokens.insert(self.supply.to_le_bytes()[0], env::predecessor_account_id());
```

## 2. No elimilating the approve rights after transfer, which leads to the approvers can do whatever they want to do for the token after first approvement.

When User approve Bob the right to transfer her tokenId, Bob can transfer many times as they want instead of transfer once.

POC
[exploit_ApproveRightAlwaysHold](https://github.com/sodexx7/Rust_practices/blob/274a821bdcd388f3d2514130fa4f5887cf7612e9/vulnerable-NEAR-contract/src/lib.rs#L109)

Solution. if the approver call transfer, should remove the approve data.

```
 pub fn transfer(&mut self, id: Id, receiver: AccountId) {
        require!(
            self.tokens.get(&id).unwrap().clone() == env::predecessor_account_id()
            || self.approvals.get(&id).unwrap().clone() == env::predecessor_account_id()
            , "not owner!"
        );
        self.tokens.insert(id, receiver);
        +++ if self.approvals.get(&id).unwrap().clone() == env::predecessor_account_id() {
        +++    self.approvals.remove(&id);
        +++ }

    }
```

## 3. Anyone can change the Struct value instead of the admin

Seems anyone can get contract instance, can directly get Contract struct value and can change it.

```
let mut contract = Contract::init(admin.clone());
contract.supply = 5;
```

POC
[exploit_AnyoneCanChangeStructValue]()

Solution to check
