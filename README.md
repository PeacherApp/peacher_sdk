# SDK code for [Peacher](https://peacher.app)
[<img alt="github" src="https://img.shields.io/badge/github-Peacher/peacher_sdk?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/PeacherApp/peacher_sdk)
[<img alt="crates.io" src="https://img.shields.io/crates/v/peacher_sdk.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/peacher_sdk)
[![docs.rs](https://img.shields.io/static/v1?label=docs.rs&message=peacher_sdk&color=green&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/peacher_sdk/latest/peacher_sdk)
This repository contains the request, response, and parameter type used by the peacher API.

Additionally, this sdk houses a `SyncClient` that you may use to manage a jursidiction.

## ** WARNINGS **
- This SDK is NOT stable. 
- This SDK is poorly documented.
- I will update this crate without warning, and the peacher API will be deployed with the update.
- All types are guaranteed to be returned by the API, but not all types have respective handlers. I add them on a need-to-add basis. Contributions are welcome and appreciated!
- This SDK is tested as a major part of the API integration tests. 
- Your clients are likely to break as I begin the debugging process of external client behavior.
