# Rust multi-thread `Read-Write-Lock` demo

In this demo, we will use the `Read-Write-Lock` to solve a concurrent problem for a real case in the world:

_We got several branches in a different country in the world, they all got their own business system. Their system will do a high-frequency account balance querying and display the result in the store big screen in real-time._

_Also, each branch system will send a balance refresh event back to head-quarter office system as well. So, the head-quarter office is able to put all branches result on the centralized big screen, to confirm that they're showing the same account balance all the time._

And we have a bunch of special requirements below:

- Each donation operation needs to take a few seconds.

- During the operations happening, `all big screens` should display `BAL: donation is happening ...` which means account balance is updating rather than showing the intermediate balance. We ask for all branch screens are showing the same result in real-time.

</br>
<hr>

## Diagram

![image](https://github.com/wisonye/rust-multi-threading-read-write-lock/blob/master/activity-diagram.JPG)

</br>
<hr>

## Solution

It's a very typical `atomic operation` scene, so how do we simulate the real-world scene and solve the problem by `Rust`? 

1. Let's introduce some `concepts` in our program:

    - `InternationalCharityAccount` - Our world-wide account which will have a lot of money :)

    - `Branch` - We got several branches in a different country in the world, they all got their own business system. Below are the branches we simulate in this demo:
        - **US - New York**
        - **CN - Shang Hai**
        - **NZ - Auckland**

    - `DonationOperationThread` - Simulate a bunch of people keep making a donation into our world-wide charity account.

    - `HeadquarterOffice` - The centralized place to display all branches balance result on the big screen.


2. We use use `std::sync::RwLock` to achieve the goal: 

    _We need concurrent share readers to read the same piece of data (account balance) at the same time, but only single exclusive writer can change the data. Also, during the time of data changing, all readers need to wait until it finishes. That's what `std::sync::RwLock` offers us._

</br>
<hr>

## How to run 

1. Setup the `hot-load` environment by running:

    ```bash
    npm install
    ```

2. Run the program

    ```bash
    npm start
    ```

</br>
<hr>

## Running screenshots

![image](https://github.com/wisonye/rust-multi-threading-read-write-lock/blob/master/screen-shot-1.png)

</br>

![image](https://github.com/wisonye/rust-multi-threading-read-write-lock/blob/master/screen-shot-2.png)

