import { api } from './api.ts'
import { ControlPanel } from './ControlPanel.tsx'
import type { User } from './types.ts'
import { Login } from './views/Login.tsx'
import { useEffect, useState } from 'react'

export function App() {
  const [user, setUser] = useState<User | null>()

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

  return <ControlPanel onLogout={logout} user={user} />
}
