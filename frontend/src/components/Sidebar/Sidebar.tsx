import type { View } from '../../types.ts'
import './Sidebar.css'

type Props = {
  current:  View
  isAdmin:  boolean
  onSelect: (view: View) => void
}

const ITEMS: { adminOnly: boolean; label: string; view: View }[] = [
  { adminOnly: false, label: 'Boards',  view: 'boards' },
  { adminOnly: false, label: 'Members', view: 'members' },
  { adminOnly: false, label: 'Teams',   view: 'teams' },
  { adminOnly: true,  label: 'Users',   view: 'users' },
]

export function Sidebar({ current, isAdmin, onSelect }: Props) {
  return (
    <aside className="sidebar">
      <p className="sidebar__brand">plaza</p>
      <nav>
        <ul className="sidebar__nav">
          {ITEMS.filter((item) => isAdmin || !item.adminOnly).map((item) => (
            <li key={item.view}>
              <button
                aria-current={item.view === current ? 'page' : undefined}
                className="sidebar__link"
                onClick={() => onSelect(item.view)}
                type="button"
              >
                {item.label}
              </button>
            </li>
          ))}
        </ul>
      </nav>
    </aside>
  )
}
