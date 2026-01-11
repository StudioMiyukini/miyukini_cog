// Re-exports pour faciliter l'import
export { createClient as createBrowserClient, getSupabaseClient } from './client'
export { createClient as createServerClient } from './server'

// Types
export type {
  Database,
  Tables,
  TablesInsert,
  TablesUpdate,
  Enums,
  Profile,
  ProfileInsert,
  ProfileUpdate,
  UserConsent,
  UserConsentInsert,
  UserConsentUpdate,
  Category,
  CategoryInsert,
  CategoryUpdate,
  UserCategoryPreference,
  UserCategoryPreferenceInsert,
  UserCategoryPreferenceUpdate,
  UserRole,
  UserTier,
  ConsentType,
} from './database.types'

export { Constants } from './database.types'
