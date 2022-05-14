use thalo::{
    aggregate::{Aggregate, TypeId},
    include_aggregate,
};

include_aggregate!("BankAccount");

#[derive(Aggregate, Clone, Debug, Default, PartialEq, TypeId)]
pub struct BankAccount {
    id: String,
    opened: bool,
    balance: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
    AccountAlreadyOpened,
    AccountNotOpen,
    InsufficientBalance,
    NegativeOrZeroAmount,
}

impl BankAccountCommand for BankAccount {
    type Error = Error;

    fn open_account(
        &self,
        initial_balance: f64,
    ) -> std::result::Result<OpenedAccountEvent, Self::Error> {
        if initial_balance < 0.0 {
            return Err(Error::NegativeOrZeroAmount);
        }

        if self.opened {
            return Err(Error::AccountAlreadyOpened);
        }

        Ok(OpenedAccountEvent { initial_balance })
    }

    fn withdraw_funds(&self, amount: f64) -> std::result::Result<WithdrewFundsEvent, Self::Error> {
        if !self.opened {
            return Err(Error::AccountNotOpen);
        }

        if amount <= 0.0 {
            return Err(Error::NegativeOrZeroAmount);
        }

        let new_balance = self.balance - amount;
        if new_balance < 0.0 {
            return Err(Error::InsufficientBalance);
        }

        Ok(WithdrewFundsEvent { amount })
    }

    fn deposit_funds(&self, amount: f64) -> std::result::Result<DepositedFundsEvent, Self::Error> {
        if !self.opened {
            return Err(Error::AccountNotOpen);
        }

        if amount <= 0.0 {
            return Err(Error::NegativeOrZeroAmount);
        }

        Ok(DepositedFundsEvent { amount })
    }
}

fn apply(bank_account: &mut BankAccount, event: BankAccountEvent) {
    use BankAccountEvent::*;

    match event {
        OpenedAccount(OpenedAccountEvent { initial_balance }) => {
            bank_account.opened = true;
            bank_account.balance = initial_balance;
        }
        DepositedFunds(DepositedFundsEvent { amount }) => {
            bank_account.balance += amount;
        }
        WithdrewFunds(WithdrewFundsEvent { amount }) => {
            bank_account.balance -= amount;
        }
    }
}
