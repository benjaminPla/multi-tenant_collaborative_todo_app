# Authorization

Date: 2026-11-01

## Options

**PxP at route level, default deny (chosen)**
- PEP: `route_layer` on the app, looks up `(method, matched path)` in `interface/authz/registry.rs`. No entry is denied (403).
- Each route is `Public` or `Guarded { action, loader }`; the loader builds the resource from path params.
- PDP: pure `decide(subject, action, resource) -> Permit(scope) | Deny` in `domain/authz/pdp.rs`.
- PIP: `infrastructure/pg_pip` loads the subject's team role.
- Handlers read `Authorized { actor, scope }`, set only by the PEP.
- Admins are permitted everything, and bypass RLS via `app.is_admin`, see [01](01-tenancy.md).
- Non-member denies on team resources return 404, so team ids can't be probed.

**PEP in each use case (rejected)**
- Opt-in: a forgotten call silently allows.

**External policy engine, e.g. OPA, Cedar, OpenFGA (rejected)**
- Extra service and policy language for two roles.

## Consequences

- New routes are denied until registered.
- Decisions see method, path and session only; body-dependent rules stay in the use case.
- One PIP query per team-scoped request.
- `teams` and `team_members` have no RLS, so the PEP is their only guard.
