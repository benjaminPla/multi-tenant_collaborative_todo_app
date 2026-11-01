export type User = {
  created_at: string
  email:      string
  id:         string
  is_admin:   boolean
}

export type View = 'boards' | 'members' | 'teams' | 'users'
