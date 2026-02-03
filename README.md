# SDK code for [Peacher](https://peacher.app)
[<img alt="github" src="https://img.shields.io/badge/github-Peacher/peacher_sdk?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/PeacherApp/peacher_sdk)
[<img alt="crates.io" src="https://img.shields.io/crates/v/peacher_sdk.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/peacher_sdk)

This repository contains the request, response, and parameter type used by the peacher API.

Additionally, this sdk houses a `SyncClient` that you may use to manage a jursidiction.

## ** WARNINGS **
- This SDK is NOT stable. 
- This SDK is poorly documented.
- I will update this crate without warning, and the peacher API will be deployed with the update.
- All types are guaranteed to be returned by the API, but not all types have respective handlers. I add them on a need-to-add basis. Contributions are welcome and appreciated!
- This SDK is tested as a major part of the API integration tests. 
- Your clients are likely to break as I begin the debugging process of external client behavior.
