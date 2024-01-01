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
    // Buy a lottery ticket
    pub fn buy_ticket(ctx: Context<Submit>) -> Result<()> {
        
        // Deserialise lottery account
        let lottery: &mut Account<Lottery> = &mut ctx.accounts.lottery;          
        let player: &mut Signer = &mut ctx.accounts.player;                 

        // First Create a system   
        // Transfer lamports to the lottery account
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &player.key(),
            &lottery.key(),
            lottery.ticket_price,
        );
        //the invoke function is a utility provided by the Anchor framework to call other Solana programs or system instructions.  
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ///When you invoke a team to perform a task, you need to provide them with the resources they need. For example, the stage setup team needs access to tools and materials. In Solana programming, when you invoke a function (like transferring lamports), you pass the necessary accounts (resources) using AccountInfo references.
                player.to_account_info(),
                lottery.to_account_info(),
            ],
        )?;

        // Deserialise ticket account
        let ticket: &mut Account<Ticket> = &mut ctx.accounts.ticket;                

        // Set submitter field as the address pays for account creation
        ticket.submitter = ctx.accounts.player.key();

        // Set ticket index equal to the counter
        ticket.idx = lottery.count;        

        // Increment total submissions counter
        lottery.count += 1;                      

        Ok(())  
    }

    ///Contexts

#[derive(Account)]    
pub struct Submit<'info>{

    #[account(init, seed, constraint, payer, bump, space)]


}

#[derive(account)]
pub struct winner {

    #[account(init, seed, constraint, seed, bump, space)]
}