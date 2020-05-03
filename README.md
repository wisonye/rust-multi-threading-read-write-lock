# Rust multi-thread `Read-Write-Lock`

This demo will simulate the `Charity Account Remote Display` case:

_Suppose we have a worldwide charity account which allows all donors to put money into it anywhere at any time. We got several branches in the `US, NZ, CN` which will show the real-time account balance in a big screen all the time._

And we have a bunch of special requirements below:

- Each donation operation needs to take around a few seconds.

- During the operations happening, we want `all big screens` to display `BAL: ......:)` which means account balance is updating rather than showing the intermediate balance. We ask for all branch screens are showing the same result in real-time.

</br>
<hr>

## Diagram

![image](https://github.com/wisonye/rust-multi-threading-read-write-lock/blob/master/activity-diagram.JPG)

</br>
<hr>

## Solution

It's a very typical `atomic operation` scene, so how do we simulate the real-world scene and solve the problem by `Rust`? 

1. Let's introduce some `concepts` in our program:

    - `InternationalCharityAccount` - Our world-wide account which have a lot of money :)

    - `Branch` - We got several branches in a different country in the world, they all got their own business system. Their system will do a high-frequency balance querying and display the current balance in their branch store big screen in real-time. Also, the system will send a balance refresh event back to head-quarter office system as well, so the head-quarter office is able to monitor all the branch balance result on the big screen in real-time. Below are the branches we simulate in this demo:
        - **US - New York**
        - **CN - Shang Hai**
        - **NZ - Auckland**

    - `DonationOperationThread` - Simulate a bunch of people make a donation into our world-wide charity account.

    - `HeadquarterOffice` - The centralized place to display all branches balance result on the big screen.


2. We use use `std::sync::RwLock` to achieve the goal: 

    _We need concurrent share readers to read the same piece of data (account balance) at the same time, but only single exclusive writer can change the data. Also, during the time of data changing, all readers need to wait until it finishes. That's what `std::sync::RwLock` offers us._


