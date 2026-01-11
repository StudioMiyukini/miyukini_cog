export { maPageCapabilityManifest } from './manifest'

export type { ProfileTypePort } from './ports/ProfileTypePort'
export type { ProfilePagePort } from './ports/ProfilePagePort'
export type { BlockRegistryPort, BlockTypeDefinition, BlockConfigValidationResult } from './ports/BlockRegistryPort'
export type { BlockDataPort, BlockDataResolver, BlockDataResolverContext } from './ports/BlockDataPort'

export type {
  ProfileType,
  ProfileTypeRule,
  ProfilePage,
  ProfileBlock,
  ProfileTag,
  ProfilePageVisibility,
  ProfileTypeDefaultVisibility,
  CreateProfileTypeInput,
  UpdateProfileTypeInput,
  CreateProfileTypeRuleInput,
  UpdateProfileTypeRuleInput,
  UpsertProfilePageInput,
  UpsertProfileBlockInput,
} from './domain/types'

export type { MaPageEventType } from './domain/events'

export { useMaPage } from './hooks/useMaPage'
export { useMaPageEvents } from './hooks/useMaPageEvents'

