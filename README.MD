# The purpose of this program is to verify the following items.
1. The calling contract defines a reference to the base contract that implements all the functions called by the caller.
1. Implement all the functions of this base contract, and implement the contract "advanced" with additional individual functions.
1. At runtime, pass the AccountId obtained by deploying the "advanced" contract to the calling Contract in the calling Contract's constructor.
1. The calling contract uses a reference to the base contract definition and the AccountId of the "advanced" contract to verify the feasibility of calling the "advanced" contract's function.

# figure
```
|----------|      |---------------|      |------------------|
|          |      |               |      |                  |
| calling  | ---> | base contract | ---> | advaned contract |
| contract |      | referrence    |      | entity           |
|          |      |               |      |                  |
|----------|      |---------------|      |------------------|
```

# Conclusion
- Using the base contract definition, I was able to successfully call the "advanced" contract's actual function.