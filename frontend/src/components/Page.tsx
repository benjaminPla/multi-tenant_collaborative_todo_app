import type { ReactNode } from 'react'

type Props = {
  action?:  string
  children: ReactNode
  title:    string
}

export function Page({ action, children, title }: Props) {
  return (
    <main className="page">
      <header className="page__header">
        <h1 className="page__title">{title}</h1>
        {action && (
          <button className="button button--primary" type="button">
            {action}
          </button>
        )}
      </header>
      {children}
    </main>
  )
}
