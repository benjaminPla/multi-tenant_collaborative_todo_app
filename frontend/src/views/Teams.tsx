import { api } from '../api.ts'
import { EmptyState } from '../components/EmptyState/EmptyState.tsx'
import { Page } from '../components/Page/Page.tsx'
import { formatDate } from '../format.ts'
import { useAction, type Loaded } from '../hooks.ts'
import type { Team } from '../types.ts'
import type { FormEvent } from 'react'

export function Teams({ teams }: { teams: Loaded<Team[]> }) {
  const action = useAction(teams.reload)

  const create = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault()
    const form = event.currentTarget
    const name = String(new FormData(form).get('name'))
    if (await action.run(() => api.teams.create(name))) {
      form.reset()
    }
  }

  return (
    <Page
      actions={
        <form className="inline-form" onSubmit={create}>
          <input aria-label="Team name" className="field__input" name="name" placeholder="Team name" required />
          <button className="button button--primary" type="submit">New team</button>
        </form>
      }
      error={action.error ?? teams.error}
      title="Teams"
    >
      {teams.data?.length === 0 && <EmptyState>No teams yet</EmptyState>}
      {!!teams.data?.length && (
        <table className="table">
          <thead>
            <tr>
              <th>Name</th>
              <th>Created</th>
              <th />
            </tr>
          </thead>
          <tbody>
            {teams.data.map((team) => (
              <tr key={team.id}>
                <td>{team.name}</td>
                <td>{formatDate(team.created_at)}</td>
                <td className="table__actions">
                  <button className="button button--danger" onClick={() => action.run(() => api.teams.delete(team.id))} type="button">
                    Delete
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </Page>
  )
}
