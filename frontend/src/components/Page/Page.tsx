import './Page.css'
import type { ReactNode } from 'react'

type Props = {
  actions?: ReactNode
  children: ReactNode
  error?:   string | null
  title:    string
}

export function Page({ actions, children, error, title }: Props) {
  return (
    <main className="page">
      <header className="page__header">
        <h1>{title}</h1>
        {actions && <div className="page__actions">{actions}</div>}
      </header>
      {error && <p className="page__error" role="alert">{error}</p>}
      {children}
    </main>
  )
}
