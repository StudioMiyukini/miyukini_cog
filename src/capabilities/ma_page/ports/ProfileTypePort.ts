import type {
  CreateProfileTypeInput,
  CreateProfileTypeRuleInput,
  ProfileType,
  ProfileTypeId,
  ProfileTypeRule,
  ProfileTypeRuleId,
  UpdateProfileTypeInput,
  UpdateProfileTypeRuleInput,
} from '../domain/types'

export type ProfileTypePort = {
  listProfileTypes(): Promise<ProfileType[]>
  getProfileTypeById(id: ProfileTypeId): Promise<ProfileType | null>
  getProfileTypeByCode(code: string): Promise<ProfileType | null>
  createProfileType(input: CreateProfileTypeInput): Promise<ProfileType>
  updateProfileType(id: ProfileTypeId, patch: UpdateProfileTypeInput): Promise<ProfileType>
  deleteProfileType(id: ProfileTypeId): Promise<void>

  listRules(profileTypeId: ProfileTypeId): Promise<ProfileTypeRule[]>
  createRule(input: CreateProfileTypeRuleInput): Promise<ProfileTypeRule>
  updateRule(id: ProfileTypeRuleId, patch: UpdateProfileTypeRuleInput): Promise<ProfileTypeRule>
  deleteRule(id: ProfileTypeRuleId): Promise<void>
}

