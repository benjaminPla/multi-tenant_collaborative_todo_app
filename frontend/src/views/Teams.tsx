import { EmptyState } from '../components/EmptyState.tsx'
import { Page } from '../components/Page.tsx'

export function Teams() {
  return (
    <Page action="New team" title="Teams">
      <EmptyState>No teams yet</EmptyState>
    </Page>
  )
}
