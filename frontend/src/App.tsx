import { api } from './api.ts'
import { Sidebar } from './components/Sidebar.tsx'
import { Topbar } from './components/Topbar.tsx'
import type { User, View } from './types.ts'
import { Boards } from './views/Boards.tsx'
import { Login } from './views/Login.tsx'
import { Members } from './views/Members.tsx'
import { Teams } from './views/Teams.tsx'
import { Users } from './views/Users.tsx'
import { useEffect, useState, type ComponentType } from 'react'

const VIEWS: Record<View, ComponentType> = {
  boards:  Boards,
  members: Members,
  teams:   Teams,
  users:   Users,
}

export function App() {
  const [user, setUser] = useState<User | null>()
  const [view, setView] = useState<View>('teams')

  useEffect(() => {
    let active = true
    api.me()
      .then((me) => active && setUser(me))
      .catch(() => active && setUser(null))
    return () => {
      active = false
    }
  }, [])

  if (user === undefined) {
    return null
  }
  if (user === null) {
    return <Login onLogin={setUser} />
  }

  const logout = async () => {
    await api.logout()
    setUser(null)
  }

  const Current = VIEWS[view]
  return (
    <div className="shell">
      <Sidebar current={view} isAdmin={user.is_admin} onSelect={setView} />
      <Topbar email={user.email} onLogout={logout} />
      <Current />
    </div>
  )
}
