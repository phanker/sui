module owned_no_tto::identified_payment {
    use sui::sui::SUI;
    use sui::coin::{Self, Coin};


    /// A `IdentifiedPayment` is an object that represents a payment with a unique
    /// identifier for that payment and what it's for for off-chain tracking purposes.
    struct IdentifiedPayment has key {
        id: UID, 
        payment_id: u64,
        coin: Coin<SUI>,
    }

    entry fun make_payment(payment_id: u64, coin: Coin<SUI>, to: address, ctx: &mut TxContext) {
        let identified_payment = IdentifiedPayment {
            id: object::new(ctx),
            payment_id,
            coin,
        };
        transfer::transfer(identified_payment, to);
    }
}
