# Rust multi-thread `Read-Write-Lock`

This demo will simulate the `Charity Account Remote Display` case:

_Suppose we have a worldwide charity account which allows all donors to put money into it anywhere at any time. We got several branches in the `US, NZ, CN` which will show the real-time account balance in a big screen all the time._

And we have a bunch of special requirements below:

- Each operations below will need around few seconds:

    - Somebody donates.
    - Internal financial team withdraw the balance into board account.

- During the operations happening, we want to display `BAL: updating` rather than showing the intermediate balance, as several operations will happen on the same time.

</br>
<hr>

## Solution

It's a very typical `atomic operation` scene, so do we simulate the real-world scene and solve the problem by `Rust`? 

1. Let's introduce some `actors` in our program:

    - `InternationalCharityAccount` - Our world-wide account which have a lot of money :)

    - `BranchDisplayThread` - Represent the specified branch to display the current charity account balance in a super big screen. We will have several of them in different place:
        - **US - New York**
        - **CN - Shang Hai**
        - **NZ - Auckland**

    - `DonorsOperationThread` - Simulate a bunch of people make a donation into our world-wide charity account.

    - `CentralizedConsole` - A centralized console to print out all branches big screen content.


2. We use use `std::sync::RwLock` to achieve the goal: 

    _We need many readers to read the same piece of data (account balance) at the same time, but only one writer can change the data. Also, during the time of data changing, all readers need to wait until it finishes. That's what `std::sync::RwLock` offers us._
