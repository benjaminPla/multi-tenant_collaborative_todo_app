import type { Team } from '../types.ts'

type Props = {
  email:        string
  onLogout:     () => void
  onTeamChange: (id: string) => void
  teamId:       string | null
  teams:        Team[]
}

export function Topbar({ email, onLogout, onTeamChange, teamId, teams }: Props) {
  return (
    <header className="topbar">
      {teams.length > 0 && (
        <select
          aria-label="Team"
          className="field__input topbar__team"
          onChange={(event) => onTeamChange(event.target.value)}
          value={teamId ?? ''}
        >
          {teams.map((team) => (
            <option key={team.id} value={team.id}>
              {team.name}
            </option>
          ))}
        </select>
      )}
      <span className="topbar__user">{email}</span>
      <button className="button" onClick={onLogout} type="button">
        Sign out
      </button>
    </header>
  )
}
