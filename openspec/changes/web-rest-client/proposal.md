## Why

There is already a REST endpoint created by the previous specs:
* openspec/specs/create-user-endpoint/spec.md
* openspec/specs/login-endpoint/spec.md
* openspec/specs/get-user-info-endpoint/spec.md

The user needs a web access to these endpoints in order to use the feature.

## What Changes

Create a new web application to access these endpoints.

## Capabilities

### New Capabilities
- `UI-create-user`: a simple web page that will allow the creation of users, providing all the necessary information like username, password, first_name, last_name, email, title, and hobby. Have a "create user" button that sends POST request to `/api/users` with the provided username, password, first_name, last_name, email, title, and hobby. If it succeeds, it will receive an `{user_id}` and redirect the user to the `UI-user-info` page.
- `UI-login`: a simple web page that will allow the login of existing users, providing all the necessary information like username and password. Have a "login" button that sends POST request to `/api/login` with the provided username and password. If it succeeds, it will receive an `{user_id}` and redirect the user to the `UI-user-info` page.
- `UI-user-info`: a simple web page that will display the information about the user, received as a result from a POST request sent to `/api/users/{user_id}` where the `{user_id}` is the result from the two previous calls, `UI-create-user` or `UI-login`

### Modified Capabilities
- none

## Impact

A new web application written in typescript that the user can access through a web interface.
