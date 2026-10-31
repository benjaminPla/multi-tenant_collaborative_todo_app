# Take-Home Assignment: Multi-Tenant Collaborative Todo App

## Goal

Build a simple **multi-tenant collaborative todo application** — inspired by Trello — where multiple organizations (tenants) can each have their own boards, and users from the same tenant can **see updates in real time** when others make changes.

This challenge is meant to assess how you:

- Design and structure multi-tenant systems
- Handle **real-time collaboration** safely and efficiently
- Build a clear, maintainable API and/or frontend
- Balance simplicity with solid engineering choices

## Requirements

### Core Functionality

1. **Multi-Tenancy**
   - Each tenant (organization/team) has isolated data.
   - Users belong to one or more tenants.
   - A user should never access data from another tenant.
2. **User Authentication**
   - Basic signup/login (session-based or JWT).
   - Authenticated users can only access their tenant's data.
3. **Boards & Projects**
   - Each tenant has multiple boards (or projects).
   - Boards contain multiple todos/tasks.
4. **Todos**
   - Each todo has a title, description, status (`todo`, `in-progress`, `done`), and optional assignee.
   - CRUD operations on todos.
5. **Real-Time Collaboration**
   - When one user adds, edits, or deletes a todo, **other users viewing the same board should see updates instantly**.
   - You may use **WebSockets**, **Server-Sent Events (SSE)**, or any other real-time mechanism.

## Technical Requirements

### Backend

- REST or GraphQL API.
- Use any language or framework you're comfortable with (e.g., Node.js/Express/NestJS, Django, Rails, FastAPI, etc.).
- Database: Postgres, SQLite, or similar.
- Implement **multi-tenancy** properly (schema-based or row-level separation).
- Implement **authentication** and **authorization**.
- Support **real-time updates** (e.g., Socket.io, WebSocket API, SSE, etc.).

### Frontend (if full-stack)

- Use any modern framework (React, Vue, Svelte, etc.).
- Authenticate a user and join a board.
- Show todos on that board.
- When another user updates a todo, the changes should appear live (no refresh required).
- Don't worry about design polish — usability and correctness matter most.

## What We're Looking For

We're evaluating:

- **System architecture & reasoning**
- **Data modeling** for multi-tenant separation
- **Real-time update handling**
- Clean, maintainable, and well-structured code
- Error handling and validation
- Security considerations (auth, tenant isolation)
- Communication clarity (via README, comments, commit messages)

## Deliverables

1. A link to your repo (GitHub, GitLab, etc.)
2. A `README.md` including:
   - How to run the app (backend and frontend)
   - How real-time updates are implemented
   - Overview of architecture and design decisions
   - Any trade-offs or things you'd improve with more time

Make setup easy via:

- `docker-compose up`, or
- a few simple setup commands.

## Time Expectation

Please spend **no more than 8–10 hours** total.
We're not expecting production-grade polish — focus on correctness, clarity, and technical thinking.

## Bonus Points

- Unit/integration tests
- Strong typing (TypeScript or static types)
- Clean architecture (API layer, service layer, domain logic)
- CI pipeline (GitHub Actions, etc.)
- Proper optimistic UI updates on the frontend
- Use of event-driven patterns (pub/sub, change streams, etc.)

## Example Scenario

- Tenant "Acme Corp" has Alice and Bob.
- Both open the "Website Launch" board in their browsers.
- Alice adds a new todo "Deploy to production."
- Bob instantly sees it appear on his screen without refreshing.

## Notes

This is **not** about building a perfect production app — it's about seeing how you approach problems, structure your code, and make design decisions.
Focus on clarity, maintainability, and thoughtful trade-offs.

Good luck and have fun!
