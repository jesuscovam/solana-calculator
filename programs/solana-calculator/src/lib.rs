use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_calculator {

    use anchor_lang::solana_program::entrypoint::ProgramResult;


    use super::*;

    pub fn create(ctx:Context<Create>, init_message: String) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }

    pub fn addition(ctx:Context<Addition>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.add(num1, num2);
        Ok(())
    }

    pub fn subtract(ctx:Context<Subtract>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.subtract(num1, num2);
        Ok(())
    }

    pub fn multiply(ctx:Context<Multiply>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.multiply(num1, num2);
        Ok(())
    }

    pub fn divide(ctx:Context<Divide>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.divide(num1, num2);
        Ok(())
    }
}

#[account]
pub struct Calculator {
    pub greeting: String,
    pub result: i64,
    pub remainder: i64,
}

impl Calculator {
    fn multiply(&mut self, num1: i64, num2:i64) {
        self.result = num1 * num2;
    }

    fn add(&mut self, num1: i64, num2:i64) {
        self.result = num1 + num2;
    }

    fn subtract(&mut self, num1: i64, num2:i64) {
        self.result = num1 - num2;
    }

    fn divide(&mut self, num1: i64, num2:i64) {
        self.result = num1 / num2;
        self.remainder = num1 % num2;
    }
}

#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>
}

#[derive(Accounts)]
pub struct Subtract<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>
}

#[derive(Accounts)]
pub struct Multiply<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>
}

#[derive(Accounts)]
pub struct Divide<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 8 + 64 + 64 + 64 + 64)]
    pub calculator: Account<'info, Calculator>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}