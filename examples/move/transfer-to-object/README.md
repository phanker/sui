To understand better how transfer-to-object may be used, we're going to look at
some possible ways of creating a cash register that can accept payments a couple different ways, and
from that we can get a better sense of how transfer-to-object can be useful in these cases.

## Representing Payments

Before getting started, we first will define a common way of making payments.
The definition of this can be found in the `common` Move package. Each identified payment
is an object that consists of a `payment_id` which is a unique identifier for
the payment (i.e., a way of tracking what the payment was for).

```
/// A unique payment for a good or service that can be uniquely identified by
/// `payment_id`.
struct IdentifiedPayment has key {
    // Object ID
    id: UID, 
    // The unique id for the good/service being paid for.
    payment_id: u64,
    // The payment
    coin: Coin<SUI>,
}
```

Customers can then make a payment with given payment ID to an address with `fun
make_payment(payment_id: u64, coin: Coin<SUI>, to: address)` this will create
an `IdentifiedPayment` and send it to the `to` address.

With how we will represents payments out of the way, lets now take a look at a
couple different ways that you could represent a cash register or perform
customer-to-business type transactions on-chain.

## Implementation 1: Using an account address

In this scenario there isn't much to implement. If you run Bill's Burgers,
you'll have an address `A` on-chain and when you take an order, you give the
customer the payment id they should use and how much they should pay, they then
send you an `IdentifiedPayment`, and you track this on your end and mark the
bill as paid when you see the `IdentifiedPayment`.

This is a very simple representation, however it has issues:
1. If Bill's Burgers private key(s) for `A` are compromised it would need to
   change it's address. This could cause issues for customers that are still
   using the older address for the business.
2. If Bill's Burgers wants to permit multiple employees to access the cash
   register it can only do via a multi-sig policy. However this could present
   issues if an employee departs, or if there are a large number of employees
   that Bill's Burgers wants to allow to access payments.

## Implementation 2: Using a shared object

To get around these issues Bill's Burgers could use a shared object. You can
see how this might be implemented here: <LINK>.

By using a shared object Bill's Burgers solves the two issues above since:
1. If Bill's Burgers private key(s) are compromised it can simply create a new
   address and change the "owner" field of the shared `Register` object to that
   new account address.
2. Bill's Burgers can add additional employees to the `Register`s
   `authorized_employees` list. If an employee departs they can easily be
   removed from this list as well without changing the object ID of the shared
   `Register` object.

However, with the shared `Register` payments need to be made a different way
than by simply transferring the coins to the shared object -- in particular
without transfer-to-object a payment to the `Register` object would involve
taking the shared `Register` object for the business, and adding the payment as
a dynamic object field:

```
public fun make_shared_payment(register_uid: &mut UID, payment_id: u64, coin: Coin<SUI>, ctx: &mut TxContext) {
    let identified_payment = IdentifiedPayment {
        id: object::new(ctx),
        payment_id,
        coin,
    };
    dynamic_object_field::add(register_uid, payment_id, identified_payment)
}
```

Because of this, if Bill's Burgers becomes incredibly popular across all their
locations and they need to server hundreds or thousands of customers at once
across all their locations, those customers payments must all be processed
serially due to the presence of the shared object. This could lead to serious
contention over the `Register` object and payments could take a while to
process. Whereas with Implementation 1 since it is using only owned objects, all
payments across all of the Bill's Burgers locations could be processed in
parallel.

Luckily, transfer-to-object can help parallelize the payment process to the
`Register` object, while also keeping the benefits of dynamic authorization and
stable interaction IDs in this implementation. Lets take a look at exactly how
it does this in the next example.

## Implementation 3: Using a shared object + transfer-to-object

With transfer-to-object, we can get the benefits of both implementations:
* The object ID stability of the shared object, and the ability to transfer the ownership of the object in case of key compromise.
* Easy way of dynamically adding, removing, and enforcing permissions on who can withdraw payments.
* Payments can all still be made using the `make_payment` function that uses
  `sui::transfer::transfer` under the hood, so payments can happen in parallel
  across all Bill's Burgers locations without needing to be sequenced against
  the shared `Register` object for Bill's Burgers.

You can see the entire implementation for the shared object register using
transfer-to-object here: <LINK>. But let's go through this and see what
changes, and doesn't change from the above two implementations.

### Interaction Stability: Object ID Remain the same

To make a payment, nothing changes from Implementation 1. In particular,
customers will still use `make_payment` and simply set the address they want to
send to to be the object ID of the Bill's Burgers `Register` object. If Bill's
Burgers changes the ownership of the `Register` object this will be totally
opaque to the customers -- they will always send their payment to the same
`Register` object.

### Receiving Payments

At a high level, handling payments after they have been made using
transfer-to-object resides somewhere between both Implementation 1, and
Implementation 2. In particular:
* Similar to Implementation 1, the object IDs of the payments you want to handle in that transaction will show up in the transaction's inputs;
* Similar to Implementation 2, there are dynamic checks that are enforced on being able to access the sent payments.

To really see what's going on here though it's best to go through the implementation of `handle_payment`:

```
/// We take the `Register` shared object mutably, along with a "ticket"
// `handle_payment` that we can exchange for the actual `IdentifiedPayment` object
// that it is associated with.
public fun handle_payment(register: &mut Register, handle_payment: Receiving<IdentifiedPayment>, ctx: &TxContext): IdentifiedPayment {
    // If the sender of the transaction that wants to handle this payment is in the list of authorized employees in the `Register` object
    // then we will permit them to withdraw the `IdentifiedPayment` object.
    assert!(vector::contains(&register.authorized_employees, tx_context::sender(ctx)), ENotAuthorized);
    // Authorization check succcessful -- exchange the `handle_payment` ticket
    // for the `IdentifiedPayment` robject it is associated with and return it.
    transfer::receive(&mut register.id, handle_payment)
}
```
