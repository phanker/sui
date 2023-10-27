module common::identified_payment {
    use sui::sui::SUI;
    use sui::coin::{Self, Coin};
    use sui::object::{Self, UID};
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};
    use sui::event;
    use sui::dynamic_object_field;

    /// An `IdentifiedPayment` is an object that represents a payment that has
    /// been made for a specific good or service that is identified by a
    /// `payment_id` that is unique to the good or service provided for the customer.
    /// NB: This has the `store` ability to allow the `make_shared_payment`
    ///     function. Without this `IdentifiedPayment` could be `key` only and
    ///     custom transfer rules can be written for it.
    struct IdentifiedPayment has key, store {
        id: UID, 
        payment_id: u64,
        coin: Coin<SUI>,
    }

    /// Event emitted when a payment is made. This contains the `payment_id`
    /// that the payment is being made for, the `payment_amount` that is being made,
    /// and the `originator` of the payment.
    struct PaymentEvent has copy, drop {
        payment_id: u64,
        payed_to: address,
        payment_amount: u64,
        originator: address,
    }

    /// Make a payment with the given payment ID to the provided `to` address.
    /// Will create an `IdentifiedPayment` object that can be unpacked by the
    /// recipient, and also emits an event.
    public fun make_payment(payment_id: u64, coin: Coin<SUI>, to: address, ctx: &mut TxContext) {
        let payment_amount = coin::value(&coin);
        let identified_payment = IdentifiedPayment {
            id: object::new(ctx),
            payment_id,
            coin,
        };
        event::emit(PaymentEvent {
            payment_id,
            payed_to: to,
            payment_amount,
            originator: tx_context::sender(ctx),
        });
        transfer::transfer(identified_payment, to);
    }

    /// Only needed for the non transfer-to-object-based cash register.
    public fun make_shared_payment(register_uid: &mut UID, payment_id: u64, coin: Coin<SUI>, ctx: &mut TxContext) {
        let payment_amount = coin::value(&coin);
        let identified_payment = IdentifiedPayment {
            id: object::new(ctx),
            payment_id,
            coin,
        };
        event::emit(PaymentEvent {
            payment_id,
            payed_to: object::uid_to_address(register_uid),
            payment_amount,
            originator: tx_context::sender(ctx),
        });
        dynamic_object_field::add(register_uid, payment_id, identified_payment)
    }

    public fun unpack(identified_payment: IdentifiedPayment): (u64, Coin<SUI>) {
        let IdentifiedPayment { id, payment_id, coin } = identified_payment;
        object::delete(id);
        (payment_id, coin)
    }
}
