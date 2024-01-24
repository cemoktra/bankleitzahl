# bankleitzahl #

bankleitzahl is a rust crate offering access to german bank data

## usage ##

```rust
// find bankleitzahlen that match on bankleitzahl
let result = Blz::from_blz("10000000");

// find bankleitzahlen that match on BIC
let result = Blz::from_bic("MARKDEF1100");
```