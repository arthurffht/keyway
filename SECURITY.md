# Security policy

## Supported versions

Keyway has not been released yet. Once it is, the latest release receives security fixes.

## Reporting a vulnerability

Please report vulnerabilities privately through
[GitHub private vulnerability reporting](https://github.com/arthurffht/keyway/security/advisories/new).
Do not open a public issue.

You will receive an acknowledgement within 72 hours. Please include the Keyway version, your
GNOME version and distribution, and the steps to reproduce.

## Scope

Keyway's threat model is described in [docs/PLAN.md §27](docs/PLAN.md#27-security-and-privacy).
Of particular interest are the engine's D-Bus interface, input handling for queries and
provider replies, and anything that could cause user data to be logged, persisted or sent
over the network.
