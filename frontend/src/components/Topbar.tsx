type Props = {
  email:    string
  onLogout: () => void
}

export function Topbar({ email, onLogout }: Props) {
  return (
    <header className="topbar">
      <span className="topbar__user">{email}</span>
      <button className="button" onClick={onLogout} type="button">
        Sign out
      </button>
    </header>
  )
}
