pub fn initialise_lottery(ctx: Context<Create>, ticket_price: u64, oracle_pubkey: Pubkey) -> Result<(), YourErrorType> {        
        ///Declare a variable then make it a reference to the lottery account and assign value     
        let mut lottery: &mut Account<Lottery> = &mut ctx.accounts.lottery;        
        lottery.authority = ctx.accounts.admin.key();                
        lottery.count = 0;           
        lottery.ticket_price = ticket_price;
        lottery.oracle = oracle_pubkey;
    
        // If an error occurs during the initialization, it will be automatically returned
        // and the following code will not be executed
        Ok(())?;
    
        // If the Ok(()) was successful, this line will be executed
        println!("Lottery initialization successful!");
    
        // If there's any other logic after the initialization, continue here...
    
        // Return Ok(()) to signal overall success
        Ok(())
    }
