export type Board = {
  created_at: string
  id:         string
  name:       string
  team_id:    string
}

export type Member = {
  created_at: string
  email:      string
  role:       Role
  user_id:    string
}

export type Role = 'member' | 'owner'

export type Team = {
  created_at: string
  id:         string
  name:       string
}

export type Todo = {
  assignee_id: string | null
  board_id:    string
  created_at:  string
  description: string
  id:          string
  status:      TodoStatus
  team_id:     string
  title:       string
  updated_at:  string
  version:     number
}

export type TodoPatch = {
  assignee_id?: string | null
  status?:      TodoStatus
  version:      number
}

export type TodoStatus = 'done' | 'in-progress' | 'todo'

export type User = {
  created_at: string
  email:      string
  id:         string
  is_admin:   boolean
}

export type View = 'boards' | 'members' | 'teams' | 'users'
