use std::u128;


/**
 * 
 * (Security analysis)
 * 
 * 
 * - Explain what the function does.
 * - What could go wrong?
 * - How to fix it?
 * 
 * Explain what the function does.
 * 1: user deposit the amount(collat) collateral_token, 
 *    calcualte the expected share token amount based on the exchange_rate 
 *    then minted the amount of shares_token  to user
 * 
 * 
 * What could go wrong? DECIMALS_SCALAR can't be any value
 * 
 *  `let amt = (collat as u128 * rate / DECIMALS_SCALAR) as u64; ` 
 *   if the value of '(collat as u128 * rate / DECIMALS_SCALAR)' greater than u64. the ultimate result will be truncated.
 *   
 *   if the desgin aims for user transfer collat, expect the share is more than collat. such as 100 collat, expecting 500 share. 
 *    should limited the DECIMALS_SCALAR value instead of any value. such as DECIMALS_SCALAR >= u64::MAX / collat*exchange_rate
 *    
 *   2: if collat as u128 * exchange_rate < DECIMALS_SCALAR, the expected share will always equal zero. 
 *    should make DECIMALS_SCALAR <=  (collat as u128 * exchange_rate)
 * 
 * 
 *     u64::MAX / collat*exchange_rate <=   DECIMALS_SCALAR  <=  (collat as u128 * exchange_rate) 
 * 
 * 
 * DECIMALS_SCALAR value is not given. It is an u128 which may have any value except 0.
 * 
 * 
 */

// Example as below
 fn _greater_than(){

   println!("u64 max {}", u64::MAX);

   let  collat: u64 = u64::MAX/1000;

   let exchange_rate:u128 =   1001; 
   let DECIMALS_SCALAR:u128 = 1;
   let amt: u128 =  (collat as u128 * exchange_rate / DECIMALS_SCALAR);
   println!("u128 {}",amt);
   println!("u64 {}",amt as u64);

 }

#[derive(Debug)]
pub enum MathError {
        ResultEqualZero,
        ResultBiggerThanU64
    }
pub type MathResult = Result<u128, MathError>;

fn calcAmt(collat:u128,DECIMALS_SCALAR:u128,exchange_rate:u128) -> MathResult {

   let amt: u128 =  (collat as u128 * exchange_rate / DECIMALS_SCALAR) as u128;

    if amt  > u64::MAX as u128 {
      return Err(MathError::ResultBiggerThanU64);
    }
    if amt == 0 {
      return Err(MathError::ResultEqualZero);
    }

    Ok(amt)

}  

fn main(){
    //  Normal
  //  let collat: u64 = u64::MAX/2;
  //  let exchange_rate:u128 =   10; 
  //  let DECIMALS_SCALAR:u128 = 100;

   // ResultBiggerThanU64
   let collat: u64 = u64::MAX;
   let exchange_rate:u128 =   10; 
   let DECIMALS_SCALAR:u128 = 1;

  // ResultEqualZero
    let collat: u64 = 100;
   let exchange_rate:u128 =   10; 
   let DECIMALS_SCALAR:u128 = u128::MAX;


   let amt =  calcAmt(collat as u128,DECIMALS_SCALAR,exchange_rate);
   println!("amt {:?}",amt);

}


 fn _panic(){

   println!("u64 max {}", u64::MAX);

   let  collat: u64 = u64::MAX;
   let exchange_rate:u128 = u64::MAX as u128;


   let exchange_rate:u128 =   10; 
   let DECIMALS_SCALAR:u128 = 1;
   let amt: u128 =  (collat as u128 * exchange_rate / DECIMALS_SCALAR) as u128;

   // deal with unexpection 
   if amt  > u64::MAX as u128 {
      panic!()
   }
   

   println!("u128 {}",amt);
   println!("u64 {}",amt as u64);


 }


// pub fn deposit(ctx: Context<Deposit>, collat: u64) -> Result<()> {
//         let rate: u128 = exchange_rate.deposit_rate as u128;
//         // collat: u64)
//         // (u128 * u128 / u128)
//         let amt = (collat as u128 * rate / DECIMALS_SCALAR) as u64; 

// 				// transfer(token, from, to, amount)
//         token::transfer(collateral_token, ctx.caller, ctx.this, collat)?;

// 				// mint_to(token, to, amount)
//         token::mint_to(shares_token, ctx.caller, amt)?;

//         Ok(())
// }


