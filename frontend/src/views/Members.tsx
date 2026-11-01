import { api } from '../api.ts'
import { EmptyState } from '../components/EmptyState/EmptyState.tsx'
import { Page } from '../components/Page/Page.tsx'
import { formatDate } from '../format.ts'
import { useAction, useLoad } from '../hooks.ts'
import type { Role } from '../types.ts'
import { useCallback, type FormEvent } from 'react'

type Props = {
  teamId: string
  userId: string
}

export function Members({ teamId, userId }: Props) {
  const members = useLoad(useCallback(() => api.members.list(teamId), [teamId]))
  const action  = useAction(members.reload)

  const add = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault()
    const form = event.currentTarget
    const data = new FormData(form)
    if (await action.run(() => api.members.add(teamId, String(data.get('email')), data.get('role') as Role))) {
      form.reset()
    }
  }

  return (
    <Page
      actions={
        <form className="inline-form" onSubmit={add}>
          <input aria-label="Email" className="field__input" name="email" placeholder="Email" required type="email" />
          <select aria-label="Role" className="field__input" name="role">
            <option value="member">member</option>
            <option value="owner">owner</option>
          </select>
          <button className="button button--primary" type="submit">Add member</button>
        </form>
      }
      error={action.error ?? members.error}
      title="Members"
    >
      {members.data?.length === 0 && <EmptyState>No members yet</EmptyState>}
      {!!members.data?.length && (
        <table className="table">
          <thead>
            <tr>
              <th>Email</th>
              <th>Role</th>
              <th>Joined</th>
              <th />
            </tr>
          </thead>
          <tbody>
            {members.data.map((member) => (
              <tr key={member.user_id}>
                <td>{member.email}</td>
                <td>
                  <select
                    aria-label={`Role of ${member.email}`}
                    className="field__input"
                    onChange={(event) => action.run(() => api.members.update(teamId, member.user_id, event.target.value as Role))}
                    value={member.role}
                  >
                    <option value="member">member</option>
                    <option value="owner">owner</option>
                  </select>
                </td>
                <td>{formatDate(member.created_at)}</td>
                <td className="table__actions">
                  <button className="button button--danger" onClick={() => action.run(() => api.members.remove(teamId, member.user_id))} type="button">
                    {member.user_id === userId ? 'Leave' : 'Remove'}
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
