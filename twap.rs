use anchor_lang::prelude::*;
use anchor_lang::Key;
use anchor_spl::token::{self, Mint, TokenAccount};
use anchor_spl::dex;
use anchor_spl::dex::serum_dex::state::OpenOrders;
use std::f64::*;
use std::time::*;
use std::convert::TryInto;



pub mod oracle{
    use super::*;
    pub fn init(ctx: Context<Init>) -> ProgramResult{
        ctx.accounts.initializer_key = *ctx.accounts.initializer.key;
        let mut timeslot =  &ctx.accounts.timestamp.unix_timestamp;
        let clock_time =  clock::Clock::get().unwrap();
        let mut cumulative = &ctx.accounts.cumulative;
        let mut count = 0;
      
        loop {
            
            let market_data = Market::load(
                &ctx.accounts.market_accounts.market,
                &dex::ID
            )?;
    
            let asks_slab = market_data.load_asks_mut(&ctx.accounts.market_accounts.asks)?;
            let bids_slab = market_data.load_bids_mut(&ctx.accounts.market_accounts.bids)?;
    
            let max_bid = match bids_slab.find_max() {
                None => self.min_ticks[0],
                Some(max_bid_index) => bids_slab
                    .get(max_bid_index)
                    .unwrap()
                    .as_leaf()
                    .unwrap()
                    .price()
                    .into(),
            };
    
            let min_ask = match asks_slab.find_min() {
                // If no orders are present, start from first user's max tick
                None => self.max_ticks[0],
                Some(min_ask_key) => asks_slab
                    .get(min_ask_key)
                    .unwrap()
                    .as_leaf()
                    .unwrap()
                    .price()
                    .into(),
            };
                let mut spot_price = (min_ask + max_bid) / 2;
                let x: f64 = 1.0001;
                let mut a_t = log(spot_price)/log(x) as f64;
                *cumulative += a_t;

                count += 1;
                if count >= 10{
                    break;
                }
    }

    pub fn twap(ctx: Context<Market>, timeslot: u64, timestamp: u64)-> ProgramResult{
        
       // let mut cumulative = cumulative + a_t;
       let mut timeslot =  &ctx.accounts.timestamp.unix_timestamp;
       &mut ctx.accounts.cumulative = 0;
       &mut ctx.accounts.calculated_twap = 0;
      
   
           //let mid_price = (min_ask + max_bid) / 2;
           //let x: f64 = 1.0001;
           //let mut at0 = logx(mid_price);
           
           let mut current_timestamp = Clock::get()?.unix_timestamp;
           for time in 0..timeslot{
               current_timestamp += timeslot;
           }
    }
    pub fn new_twap(spot_price: u64, twap: u64) -> u64{

    }
    pub fn previous_timestamp(new_twap_time: u64, unix_timestamp: i64) -> u64 {
        return std::cmp::min(unix_timestamp.try_into().unwrap(), new_twap_time);
    }
        /// Serum crank function to synchronize OpenOrders with completed order data
    pub fn update_open_orders(ctx: &Context<Crank>) -> ProgramResult {
        let consume_events_instr = dex::serum_dex::instruction::consume_events(
            &dex::ID,
            vec![ctx.accounts.open_orders.key],
            &ctx.accounts.market_accounts.market.key,
            &ctx.accounts.market_accounts.event_queue.key,
            &ctx.accounts.coin_wallet.key(),
            &ctx.accounts.pc_wallet.key(),
            10
        )?;

        let consume_events_accounts: &[anchor_lang::prelude::AccountInfo<'_>] = &[
    
            ctx.accounts.open_orders.clone(),
            ctx.accounts.market_accounts.market.clone(),
            ctx.accounts.market_accounts.event_queue.clone(),
            ctx.accounts.coin_wallet.to_account_info().clone(),
            ctx.accounts.pc_wallet.to_account_info().clone()
        ];

        anchor_lang::solana_program::program::invoke(
            &consume_events_instr,
            consume_events_accounts
        )
}
    pub fn refresh_orders_crank(&mut self, ctx: Context<Crank>) -> ProgramResult {
        


        
        max_bid_tick = max_bid + spread / 2;
        min_ask_tick = if spread % 2 == 0 {
        min_ask - spread / 2 + 1
    } else {
        min_ask - spread / 2
    };

        let order_width = ORDER_WIDTH as u64;

    // TODO fix edge cases
        min_bid_tick = if max_bid_tick > order_width {
        max_bid_tick - order_width + 1
    } else {
        1
    };
        max_ask_tick = min_ask_tick + order_width - 1;


    // Discard ticks where orders have already been placed
    // TODO refresh partially executed orders
    // TODO resolve edge cases
        if self.start_tick_for_bids != 0 {
            let price_movement = (min_bid_tick - self.start_tick_for_bids) as i32;

        if price_movement > 0 {
            min_ask_tick = max_ask_tick + 1;
            max_ask_tick += price_movement as u64;

            // TODO remove bids from start

        } else if price_movement < 0 {
            max_bid_tick = min_bid_tick - 1;
            min_bid_tick -= price_movement as u64;

            // TODO remove asks from end

        } else {
            // Find difference between placed orders and fulfillment
            // If it exceeds 30%, find liquidity at this tick and place new order
            // But for correct liquidity, orders must be settled on the other side
            msg!("No change in prices, exiting");

            return Ok(());
        }
    }
}        
            
        }
    }



#[derive(Accounts)]
pub struct Init<'info>{       
    pub initializer: UncheckedAccount<'info>,
    #[account(mut)]
    calculated_twap: u64,
    #[account(init, payer = initializer, bump = )]
    spot_price: u64, 
    #[account(init, )]
    initialization_timestamp: Sysvar<'info, Clock>,
    pub market: Account<'info, Market>,
    timestamp: Sysvar<'info, Clock>,        
}


#[derive(Accounts)]
pub struct Modifytwap<'info>{
    #[account(mut)]
    pub market: UncheckedAccount,
    pub 
}

#[derive(Accounts)]
pub struct FindOrderZone<'info> {
    market_accounts: Market<'info>,
}

#[derive(Accounts)]
pub struct Crank<'info> {
    market_accounts: Market<'info>,

    authority: UnchekedcAccount<'info>,

    /// For orders and settlement
    #[account(mut)]
    coin_wallet: Account<'info, TokenAccount>,
    #[account(mut)]
    pc_wallet: Account<'info, TokenAccount>,

    /// Mints
    coin_mint: Account<'info, Mint>,
    pc_mint: Account<'info, Mint>,

    #[account(mut)]
    // CpiAccount not supported
    open_orders: UncheckedAccount<'info>,

    /// Programs
    token_program: UncheckedAccount<'info>,
    dex_program: UncheckedAccount<'info>,

    /// Sysvars
    rent: Sysvar<'info, Rent>,

}
#[derive(Accounts)]
pub struct Market<'info>{
    #[account(mut)]
    asks: UncheckedAccount<'info>,
    #[account(mut)]
    bids: UncheckedAccount<'info>,
    #[account(mut)]
    cumulative: u64,
    timeslot: Sysvar<'info, Clock>,
             
}
#[account]
pub struct Twap{
    
    accumulator: Box<Vec<u64>>,
    old_twap: u64,
    new_twap: u64,
    bump: u8,
}
}