# can_afford

`can_afford(gold: u64, cost: u64) -> bool`

```mermaid
flowchart TD
    A([Start]) --> B{gold >= cost?}
    B -- Yes --> T([Return true])
    B -- No --> F([Return false])
```
