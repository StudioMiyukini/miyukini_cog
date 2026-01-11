export type ProfileTypeId = string
export type ProfileTypeRuleId = string
export type ProfilePageId = string
export type ProfileBlockId = string
export type ProfileTagId = string

export type UserId = string

export type ProfilePageVisibility = 'private' | 'public' | 'unlisted'
export type ProfileTypeDefaultVisibility = 'private' | 'public'

export type ProfileType = {
  id: ProfileTypeId
  code: string
  name: string
  description: string | null
  defaultVisibility: ProfileTypeDefaultVisibility
  allowedRoles: string[] | null
  defaultBlocks: unknown | null // JSONB (liste ordonnée)
  createdBy: UserId | null
  createdAt: string
  updatedAt: string
}

export type ProfileTypeRule = {
  id: ProfileTypeRuleId
  profileTypeId: ProfileTypeId
  priority: number
  matchRole: string | null
  matchTagsAll: string[] | null
  matchTagsAny: string[] | null
  matchContextType: string | null
  matchContextId: string | null
  isActive: boolean
}

export type ProfilePage = {
  id: ProfilePageId
  userId: UserId
  profileTypeId: ProfileTypeId
  slug: string | null
  visibility: ProfilePageVisibility
  title: string | null
  summary: string | null
  tags: string[] | null
  meta: Record<string, unknown> | null
  publishedAt: string | null
  createdAt: string
  updatedAt: string
}

export type ProfileBlock = {
  id: ProfileBlockId
  pageId: ProfilePageId
  blockType: string
  position: number
  isEnabled: boolean
  config: Record<string, unknown> | null
  dataRef: Record<string, unknown> | null
  createdAt: string
  updatedAt: string
}

export type ProfileTag = {
  id: ProfileTagId
  name: string
  category: string | null
  createdAt: string
}

export type CreateProfileTypeInput = {
  code: string
  name: string
  description?: string | null
  defaultVisibility?: ProfileTypeDefaultVisibility
  allowedRoles?: string[] | null
  defaultBlocks?: unknown | null
}

export type UpdateProfileTypeInput = Partial<Omit<CreateProfileTypeInput, 'code'>> & {
  name?: string
  description?: string | null
}

export type CreateProfileTypeRuleInput = {
  profileTypeId: ProfileTypeId
  priority: number
  matchRole?: string | null
  matchTagsAll?: string[] | null
  matchTagsAny?: string[] | null
  matchContextType?: string | null
  matchContextId?: string | null
  isActive?: boolean
}

export type UpdateProfileTypeRuleInput = Partial<Omit<CreateProfileTypeRuleInput, 'profileTypeId'>> & {
  priority?: number
}

export type UpsertProfilePageInput = {
  userId: UserId
  profileTypeId: ProfileTypeId
  slug?: string | null
  visibility?: ProfilePageVisibility
  title?: string | null
  summary?: string | null
  tags?: string[] | null
  meta?: Record<string, unknown> | null
}

export type UpsertProfileBlockInput = {
  pageId: ProfilePageId
  blockType: string
  position: number
  isEnabled?: boolean
  config?: Record<string, unknown> | null
  dataRef?: Record<string, unknown> | null
}

