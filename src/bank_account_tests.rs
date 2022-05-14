use thalo_testing::*;

use crate::bank_account::{
    BankAccount, BankAccountCommand, DepositedFundsEvent, Error, OpenedAccountEvent,
    WithdrewFundsEvent,
};

const ACCOUNT_NAME: &str = "omprakash.sridharan";

#[test]
fn open_account() {
    BankAccount::given_no_events(ACCOUNT_NAME.to_string())
        .when(|bank_account| bank_account.open_account(-100.0))
        .then_err(Error::NegativeOrZeroAmount)
        .when(|bank_account| bank_account.open_account(10.0))
        .then_ok(OpenedAccountEvent {
            initial_balance: 10.0,
        })
        .apply()
        .when(|bank_account| bank_account.open_account(1.0))
        .then_err(Error::AccountAlreadyOpened);
}

#[test]
fn deposit_funds() {
    BankAccount::given_no_events(ACCOUNT_NAME.to_string())
        .when(|bank_account| bank_account.deposit_funds(10.0))
        .then_err(Error::AccountNotOpen)
        .when(|bank_account| bank_account.open_account(0.0))
        .then_ok(OpenedAccountEvent {
            initial_balance: 0.0,
        })
        .apply()
        .when(|bank_account| bank_account.deposit_funds(10.0))
        .then_ok(DepositedFundsEvent { amount: 10.0 })
        .apply()
        .when(|bank_account| bank_account.deposit_funds(0.0))
        .then_err(Error::NegativeOrZeroAmount)
        .when(|bank_account| bank_account.deposit_funds(-50.0))
        .then_err(Error::NegativeOrZeroAmount);
}

#[test]
fn withdraw_funds() {
    BankAccount::given_no_events(ACCOUNT_NAME.to_string())
        .when(|bank_account| bank_account.withdraw_funds(10.0))
        .then_err(Error::AccountNotOpen)
        .when(|bank_account| bank_account.open_account(0.0))
        .then_ok(OpenedAccountEvent {
            initial_balance: 0.0,
        })
        .apply()
        .when(|bank_account| bank_account.withdraw_funds(10.0))
        .then_err(Error::InsufficientBalance)
        .when(|bank_account| bank_account.deposit_funds(50.0))
        .then_ok(DepositedFundsEvent { amount: 50.0 })
        .apply()
        .when(|bank_account| bank_account.withdraw_funds(45.0))
        .then_ok(WithdrewFundsEvent { amount: 45.0 })
        .apply()
        .when(|bank_account| bank_account.withdraw_funds(6.0))
        .then_err(Error::InsufficientBalance)
        .when(|bank_account| bank_account.withdraw_funds(0.0))
        .then_err(Error::NegativeOrZeroAmount)
        .when(|bank_account| bank_account.withdraw_funds(-20.0))
        .then_err(Error::NegativeOrZeroAmount);
}
