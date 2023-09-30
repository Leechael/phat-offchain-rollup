#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(Ownable, AccessControl)]
#[openbrush::contract]
pub mod test_contract {

    use ink::env::debug_println;
    use openbrush::contracts::access_control::*;
    use openbrush::contracts::ownable::*;
    use openbrush::traits::Storage;
    use phat_rollup_anchor_ink::traits::meta_transaction::{self, *};
    use phat_rollup_anchor_ink::traits::rollup_anchor::{self, *};

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct MyContract {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        access: access_control::Data,
        #[storage_field]
        rollup_anchor: rollup_anchor::Data,
        #[storage_field]
        meta_transaction: meta_transaction::Data,
    }

    impl MyContract {
        #[ink(constructor)]
        pub fn new(phat_attestor: AccountId) -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            // set the owner of this contract
            ownable::Internal::_init_with_owner(&mut instance, caller);
            // set the admin of this contract
            access_control::Internal::_init_with_admin(&mut instance, Some(caller));
            // grant the role attestor to the given address
            AccessControl::grant_role(&mut instance, ATTESTOR_ROLE, Some(phat_attestor))
                .expect("Should grant the role ATTESTOR_ROLE");
            instance
        }
    }

    impl RollupAnchor for MyContract {}
    impl MetaTransaction for MyContract {}

    impl rollup_anchor::MessageHandler for MyContract {
        fn on_message_received(&mut self, action: Vec<u8>) -> Result<(), RollupAnchorError> {
            debug_println!("Message received {:?}'", action);
            Ok(())
        }
    }

    impl rollup_anchor::EventBroadcaster for MyContract {
        fn emit_event_message_queued(&self, id: u32, data: Vec<u8>) {
            debug_println!(
                "Emit event 'message queued {{ id: {:?}, data: {:2x?} }}",
                id,
                data
            );
        }
        fn emit_event_message_processed_to(&self, id: u32) {
            debug_println!("Emit event 'message processed to {:?}'", id);
        }
    }

    impl meta_transaction::EventBroadcaster for MyContract {
        fn emit_event_meta_tx_decoded(&self) {
            debug_println!("Meta transaction decoded");
        }
    }
}
