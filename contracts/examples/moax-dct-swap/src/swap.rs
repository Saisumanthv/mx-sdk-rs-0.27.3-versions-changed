#![no_std]

dharitri_wasm::imports!();
dharitri_wasm::derive_imports!();

const MOAX_NUM_DECIMALS: usize = 18;

/// Converts between MOAX and a wrapped MOAX DCT token.
///	1 MOAX = 1 wrapped MOAX and is interchangeable at all times.
/// Also manages the supply of wrapped MOAX tokens.
#[dharitri_wasm::contract]
pub trait MoaxDctSwap {
    #[init]
    fn init(&self) {}

    // endpoints - owner-only

    #[only_owner]
    #[payable("MOAX")]
    #[endpoint(issueWrappedMoax)]
    fn issue_wrapped_moax(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        initial_supply: BigUint,
        #[payment] issue_cost: BigUint,
    ) -> AsyncCall {
        require!(
            self.wrapped_moax_token_id().is_empty(),
            "wrapped moax was already issued"
        );

        let caller = self.blockchain().get_caller();

        self.issue_started_event(&caller, &token_ticker, &initial_supply);

        self.send()
            .dct_system_sc_proxy()
            .issue_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                &initial_supply,
                FungibleTokenProperties {
                    num_decimals: MOAX_NUM_DECIMALS,
                    can_freeze: false,
                    can_wipe: false,
                    can_pause: false,
                    can_mint: true,
                    can_burn: false,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: false,
                },
            )
            .async_call()
            .with_callback(self.callbacks().dct_issue_callback(&caller))
    }

    #[callback]
    fn dct_issue_callback(
        &self,
        caller: &ManagedAddress,
        #[payment_token] token_identifier: TokenIdentifier,
        #[payment] returned_tokens: BigUint,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        // callback is called with DCTTransfer of the newly issued token, with the amount requested,
        // so we can get the token identifier and amount from the call data
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.issue_success_event(caller, &token_identifier, &returned_tokens);
                self.unused_wrapped_moax().set(returned_tokens.as_ref());
                self.wrapped_moax_token_id().set(token_identifier.as_ref());
            },
            ManagedAsyncCallResult::Err(message) => {
                self.issue_failure_event(caller, &message.err_msg);

                // return issue cost to the owner
                // TODO: test that it works
                if token_identifier.is_moax() && returned_tokens > 0 {
                    self.send().direct_moax(caller, &returned_tokens, &[]);
                }
            },
        }
    }

    #[only_owner]
    #[endpoint(mintWrappedMoax)]
    fn mint_wrapped_moax(&self, amount: BigUint) -> AsyncCall {
        require!(
            !self.wrapped_moax_token_id().is_empty(),
            "Wrapped MOAX was not issued yet"
        );

        let wrapped_moax_token_id = self.wrapped_moax_token_id().get();
        let caller = self.blockchain().get_caller();
        self.mint_started_event(&caller, &amount);

        self.send()
            .dct_system_sc_proxy()
            .mint(&wrapped_moax_token_id, &amount)
            .async_call()
            .with_callback(self.callbacks().dct_mint_callback(&caller, &amount))
    }

    #[callback]
    fn dct_mint_callback(
        &self,
        caller: &ManagedAddress,
        amount: &BigUint,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.mint_success_event(caller);
                self.unused_wrapped_moax()
                    .update(|unused_wrapped_moax| *unused_wrapped_moax += amount);
            },
            ManagedAsyncCallResult::Err(message) => {
                self.mint_failure_event(caller, &message.err_msg);
            },
        }
    }

    // endpoints

    #[payable("MOAX")]
    #[endpoint(wrapMoax)]
    fn wrap_moax(&self, #[payment] payment: BigUint) {
        require!(payment > 0u32, "Payment must be more than 0");
        require!(
            !self.wrapped_moax_token_id().is_empty(),
            "Wrapped MOAX was not issued yet"
        );

        self.unused_wrapped_moax().update(|unused_wrapped_moax| {
            require!(
                *unused_wrapped_moax > payment,
                "Contract does not have enough wrapped MOAX. Please try again once more is minted."
            );

            *unused_wrapped_moax -= &payment;
        });

        let caller = self.blockchain().get_caller();
        self.send().direct(
            &caller,
            &self.wrapped_moax_token_id().get(),
            0,
            &payment,
            b"wrapping",
        );

        self.wrap_moax_event(&caller, &payment);
    }

    #[payable("*")]
    #[endpoint(unwrapMoax)]
    fn unwrap_moax(
        &self,
        #[payment] wrapped_moax_payment: BigUint,
        #[payment_token] token_identifier: TokenIdentifier,
    ) {
        require!(
            !self.wrapped_moax_token_id().is_empty(),
            "Wrapped MOAX was not issued yet"
        );

        let wrapped_moax_token_identifier = self.wrapped_moax_token_id().get();
        require!(
            token_identifier == wrapped_moax_token_identifier,
            "Wrong dct token"
        );

        require!(wrapped_moax_payment > 0u32, "Must pay more than 0 tokens!");
        // this should never happen, but we'll check anyway
        require!(
            wrapped_moax_payment
                <= self
                    .blockchain()
                    .get_sc_balance(&TokenIdentifier::moax(), 0),
            "Contract does not have enough funds"
        );

        self.unused_wrapped_moax()
            .update(|unused_wrapped_moax| *unused_wrapped_moax += &wrapped_moax_payment);

        // 1 wrapped MOAX = 1 MOAX, so we pay back the same amount
        let caller = self.blockchain().get_caller();
        self.send()
            .direct_moax(&caller, &wrapped_moax_payment, b"unwrapping");

        self.unwrap_moax_event(&caller, &wrapped_moax_payment);
    }

    #[view(getLockedMoaxBalance)]
    fn get_locked_moax_balance(&self) -> BigUint {
        self.blockchain()
            .get_sc_balance(&TokenIdentifier::moax(), 0)
    }

    // storage

    #[view(getWrappedMoaxTokenIdentifier)]
    #[storage_mapper("wrapped_moax_token_id")]
    fn wrapped_moax_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getUnusedWrappedMoax)]
    #[storage_mapper("unused_wrapped_moax")]
    fn unused_wrapped_moax(&self) -> SingleValueMapper<BigUint>;

    // events

    #[event("issue-started")]
    fn issue_started_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] token_ticker: &ManagedBuffer,
        initial_supply: &BigUint,
    );

    #[event("issue-success")]
    fn issue_success_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] token_identifier: &TokenIdentifier,
        initial_supply: &BigUint,
    );

    #[event("issue-failure")]
    fn issue_failure_event(&self, #[indexed] caller: &ManagedAddress, message: &ManagedBuffer);

    #[event("mint-started")]
    fn mint_started_event(&self, #[indexed] caller: &ManagedAddress, amount: &BigUint);

    #[event("mint-success")]
    fn mint_success_event(&self, #[indexed] caller: &ManagedAddress);

    #[event("mint-failure")]
    fn mint_failure_event(&self, #[indexed] caller: &ManagedAddress, message: &ManagedBuffer);

    #[event("wrap-moax")]
    fn wrap_moax_event(&self, #[indexed] user: &ManagedAddress, amount: &BigUint);

    #[event("unwrap-moax")]
    fn unwrap_moax_event(&self, #[indexed] user: &ManagedAddress, amount: &BigUint);
}
