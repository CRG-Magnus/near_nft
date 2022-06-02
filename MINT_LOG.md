## MINT LOG ##

---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

# Working ref mint

items?.naramunzdev.testnet
near call $ID nft_mint '{"token_id":"1001000011","metadata":{"title":"Sword of the Undead","description":"Rumors says this sword can reanimate the Dead","media":"https://ipfs.io/ipfs/QmZ55c2oiJvkNGWArgjJNpF3tvRYdMCfT3PytBa4ABouQr?filename=1001000010.png","reference":"https://ipfs.io/ipfs/QmRRSqJ9gsTa7YMJFCjHXaKUcdbTseSBFAMJ82bWtvfRTG?filename=1001000010.json","reference_hash":"MzA4NzU3MTcyNjI0YjdhOGE1ZWY3OWQ2NDk5YmM2NGNkMDE5YmNhOWZmODllMThkODcyM2Y2ZTZjMTMwMGE0Ng=="},"receiver_id":"'$ID'","perpetual_royalties":{"magnusb.testnet":1000}}' --accountId $ID --amount 0.01

items3.paramunz.testnet
near call $ID nft_mint '{"token_id":"02","metadata":{"title":"Sword of Mass Extinction","description":"Rumors says this sword can destroy the world","media":"https://bafybeihezci6he7o7rrgokm4o7sfgwik57w76dfbwqk54afsmwsrfpyadm.ipfs.nftstorage.link/101.png","reference":"https://bafybeihezci6he7o7rrgokm4o7sfgwik57w76dfbwqk54afsmwsrfpyadm.ipfs.nftstorage.link/101.json","reference_hash":"ODBhODhjYzNlMmRlMWE3YTE5NDlhNjA5MjRmYjIwODZhYmIyMGQ1ZjkzOTdkMTMyNGY5MTFhZjllZTljOTM5ZQ=="},"receiver_id":"'naramunzdev.testnet'","perpetual_royalties":{"paramunz.testnet":800}}' --accountId $ID --amount 0.01

=> works!

---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

## items3.paramunz.testnet ##

# 01
near call $ID nft_mint '{"token_id":"01","metadata":{"title":"Sword of Mass Extinction","description":"Rumors says this sword can destroy the world","media":"https://bafybeihezci6he7o7rrgokm4o7sfgwik57w76dfbwqk54afsmwsrfpyadm.ipfs.nftstorage.link/101.png","reference":"https://bafybeihezci6he7o7rrgokm4o7sfgwik57w76dfbwqk54afsmwsrfpyadm.ipfs.nftstorage.link/101.json","reference_hash":"ODBhODhjYzNlMmRlMWE3YTE5NDlhNjA5MjRmYjIwODZhYmIyMGQ1ZjkzOTdkMTMyNGY5MTFhZjllZTljOTM5ZQ=="},"receiver_id":"'naramunzdev.testnet'","perpetual_royalties":{"paramunz.testnet":800}}' --accountId $ID --amount 0.01

OK!

# 02 BURNED
near call $ID nft_mint '{"token_id":"02","metadata":{"title":"Sword of Mass Extinction","description":"Rumors says this sword can destroy the world","media":"https://bafybeihezci6he7o7rrgokm4o7sfgwik57w76dfbwqk54afsmwsrfpyadm.ipfs.nftstorage.link/101.png","reference":"https://bafybeihezci6he7o7rrgokm4o7sfgwik57w76dfbwqk54afsmwsrfpyadm.ipfs.nftstorage.link/101.json","reference_hash":"ODBhODhjYzNlMmRlMWE3YTE5NDlhNjA5MjRmYjIwODZhYmIyMGQ1ZjkzOTdkMTMyNGY5MTFhZjllZTljOTM5ZQ=="},"receiver_id":"'naramunzdev.testnet'","perpetual_royalties":{"paramunz.testnet":800}}' --accountId $ID --amount 0.01

BURN 02 => OK! Does not vanish on paras?

# 03
near call $ID nft_mint '{"token_id":"03","metadata":{"title":"Sword of the Feeble","description":"This sword is so heavy it can barely be lifted","media":"https://bafybeif3se7a32s4se4din3fzjm7qjj3cv3vgmup4yyhn6ldzygvjrtesy.ipfs.nftstorage.link/102.png","extra":"This is a gift for a big game supporter","reference":"https://bafybeif3se7a32s4se4din3fzjm7qjj3cv3vgmup4yyhn6ldzygvjrtesy.ipfs.nftstorage.link/102.json","reference_hash":"NmI4MzM1NzdlY2Q2NzBkYzkzN2FjZTNiYTEwZmFhMDhkNWFkOGYyN2Q3ZTdiYzcwNDFiM2U5N2FhZmE1Yzk1ZA=="},"receiver_id":"'naramunzdev.testnet'","perpetual_royalties":{"paramunz.testnet":800}}' --accountId $ID --amount 0.01

OK!

# 04
near call $ID nft_mint '{"token_id":"04","metadata":{"title":"Shield of Lichln","description":"Once held by Lichln","media":"https://bafybeihxwx5whwevxmfksbxustqs3tptfftvjt6khdmi4pu26oz7hmnxga.ipfs.nftstorage.link/104.png","copies":1,"extra":"Pre-alpha NFT","reference":"https://bafybeihxwx5whwevxmfksbxustqs3tptfftvjt6khdmi4pu26oz7hmnxga.ipfs.nftstorage.link/104.json","reference_hash":"YTFjN2RjZTVkZDQ5OTJhNTY5Njg4ZjhmYTk5MTY5Yzk0Yzc2NjIwOTY4Mzc5Yjk5YTMxMTcxZmVhM2NjY2ZiNQ=="},"receiver_id":"'naramunzdev.testnet'","perpetual_royalties":{"paramunz.testnet":800}}' --accountId $ID --amount 0.01

# 05 - Test mint by another account -OK!
near call $ID nft_mint '{"token_id":"05","metadata":{"title":"Shield of Lichln 2","description":"Once held by Lichln","media":"https://bafybeihxwx5whwevxmfksbxustqs3tptfftvjt6khdmi4pu26oz7hmnxga.ipfs.nftstorage.link/104.png","copies":1,"extra":"Pre-alpha NFT","reference":"https://bafybeihxwx5whwevxmfksbxustqs3tptfftvjt6khdmi4pu26oz7hmnxga.ipfs.nftstorage.link/104.json","reference_hash":"YTFjN2RjZTVkZDQ5OTJhNTY5Njg4ZjhmYTk5MTY5Yzk0Yzc2NjIwOTY4Mzc5Yjk5YTMxMTcxZmVhM2NjY2ZiNQ=="},"receiver_id":"'naramunzdev.testnet'","perpetual_royalties":{"paramunz.testnet":800}}' --accountId naramunzdev.testnet --amount 0.01


## collectibles.paramunz.testnet ##

# 05 - 07 (3)
near call $ID nft_mint '{"token_id":"07","metadata":{"title":"Badge of Support","description":"Badge awarded for the support of Dust of Paramunz game","media":"https://bafybeiecwmrdh2jvbxjnp7r6nzwwoof4uwdy7wayakhohsxzotn6hqmr7a.ipfs.nftstorage.link/201.png","copies":3,"extra":"Pre-alpha","reference":"https://bafybeiecwmrdh2jvbxjnp7r6nzwwoof4uwdy7wayakhohsxzotn6hqmr7a.ipfs.nftstorage.link/201.json","reference_hash":"YWViMjg1MmVhZGEzN2NkZjBhYmM1Njc1MGUzMGQ2YjMwMWViOTRhZDM3YjYzMDRlNmM0M2IxMDVhMzQ1Y2UxZA=="},"receiver_id":"'naramunzdev.testnet'","perpetual_royalties":{"paramunz.testnet":800}}' --accountId $ID --amount 0.01

# 08
near call $ID nft_mint '{"token_id":"08","metadata":{"title":"Badge of Grand Support","description":"Badge awarded for the support of Dust of Paramunz game","media":"https://bafybeiadk7zmzour4ifpolu5noczjgycqw742leh7uecufuz75xpc6kh6u.ipfs.nftstorage.link/202.png","copies":3,"extra":"Pre-alpha","reference":"https://bafybeiecwmrdh2jvbxjnp7r6nzwwoof4uwdy7wayakhohsxzotn6hqmr7a.ipfs.nftstorage.link/201.json","reference_hash":"YWViMjg1MmVhZGEzN2NkZjBhYmM1Njc1MGUzMGQ2YjMwMWViOTRhZDM3YjYzMDRlNmM0M2IxMDVhMzQ1Y2UxZA=="},"receiver_id":"'naramunzdev.testnet'","perpetual_royalties":{"paramunz.testnet":800}}' --accountId $ID --amount 0.01

# 09
near call $ID nft_mint '{"token_id":"09","metadata":{"title":"Badge of Support","description":"Badge awarded for the support of Dust of Paramunz game","media":"https://bafybeiecwmrdh2jvbxjnp7r6nzwwoof4uwdy7wayakhohsxzotn6hqmr7a.ipfs.nftstorage.link/201.png","extra":"Pre-alpha","reference":"https://bafybeiecwmrdh2jvbxjnp7r6nzwwoof4uwdy7wayakhohsxzotn6hqmr7a.ipfs.nftstorage.link/201.json","reference_hash":"YWViMjg1MmVhZGEzN2NkZjBhYmM1Njc1MGUzMGQ2YjMwMWViOTRhZDM3YjYzMDRlNmM0M2IxMDVhMzQ1Y2UxZA=="},"receiver_id":"'naramunzdev.testnet'","perpetual_royalties":{"paramunz.testnet":800}}' --accountId $ID --amount 0.01

# 10 - Not minted! No miniting permission for external account
near call $ID nft_mint '{"token_id":"10","metadata":{"title":"Badge of Support","description":"Badge awarded for the support of Dust of Paramunz game","media":"https://bafybeiecwmrdh2jvbxjnp7r6nzwwoof4uwdy7wayakhohsxzotn6hqmr7a.ipfs.nftstorage.link/201.png","extra":"Pre-alpha","reference":"https://bafybeiecwmrdh2jvbxjnp7r6nzwwoof4uwdy7wayakhohsxzotn6hqmr7a.ipfs.nftstorage.link/201.json","reference_hash":"YWViMjg1MmVhZGEzN2NkZjBhYmM1Njc1MGUzMGQ2YjMwMWViOTRhZDM3YjYzMDRlNmM0M2IxMDVhMzQ1Y2UxZA=="},"receiver_id":"'naramunzdev.testnet'","perpetual_royalties":{"paramunz.testnet":800}}' --accountId paramunz.testnet --amount 0.01