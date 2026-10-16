# Queue

| Item | State | Next step |
| --- | --- | --- |
| Add egui36 and egui37 compatibility | Validated | Native/WASM, strict Clippy and palette/doctests pass on 0.36.2 and 0.37.0; publish main. |
| Retain legacy egui26 default | Validated | Default-feature check and both palette tests pass on 0.26.2. |
| Optional palette generator | Validated | Checked-in palette hash remains identical when Whiskers is absent. |
| Upstream integration | Not requested | No PR, issue, release or crates.io upload is scheduled. |
| Inherited GitHub workflows | Disabled | Do not enable Ubuntu runners; local Fedora checks are the acceptance path. |
