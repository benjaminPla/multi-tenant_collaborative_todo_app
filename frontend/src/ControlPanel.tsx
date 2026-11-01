import { api } from './api.ts'
import { EmptyState } from './components/EmptyState.tsx'
import { Page } from './components/Page.tsx'
import { Sidebar } from './components/Sidebar.tsx'
import { Topbar } from './components/Topbar.tsx'
import { useLoad } from './hooks.ts'
import type { User, View } from './types.ts'
import { Boards } from './views/Boards.tsx'
import { Members } from './views/Members.tsx'
import { Teams } from './views/Teams.tsx'
import { Users } from './views/Users.tsx'
import { useState } from 'react'

type Props = {
  onLogout: () => void
  user:     User
}

export function ControlPanel({ onLogout, user }: Props) {
  const teams               = useLoad(api.teams.list)
  const [teamId, setTeamId] = useState<string | null>(null)
  const [view, setView]     = useState<View>('teams')
  const team                = teams.data?.find((t) => t.id === teamId) ?? teams.data?.[0]

  const needsTeam = (title: string) => (
    <Page title={title}>
      <EmptyState>Create a team first</EmptyState>
    </Page>
  )

  const content = () => {
    switch (view) {
      case 'boards':
        return team ? <Boards key={team.id} teamId={team.id} /> : needsTeam('Boards')
      case 'members':
        return team ? <Members key={team.id} teamId={team.id} userId={user.id} /> : needsTeam('Members')
      case 'teams':
        return <Teams teams={teams} />
      case 'users':
        return <Users />
    }
  }

  return (
    <div className="shell">
      <Sidebar current={view} isAdmin={user.is_admin} onSelect={setView} />
      <Topbar email={user.email} onLogout={onLogout} onTeamChange={setTeamId} teamId={team?.id ?? null} teams={teams.data ?? []} />
      {content()}
    </div>
  )
}
