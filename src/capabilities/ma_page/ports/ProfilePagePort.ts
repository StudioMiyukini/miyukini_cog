import type {
  ProfileBlock,
  ProfileBlockId,
  ProfilePage,
  ProfilePageId,
  ProfilePageVisibility,
  UpsertProfileBlockInput,
  UpsertProfilePageInput,
  UserId,
} from '../domain/types'

export type ProfilePagePort = {
  getMyPage(): Promise<ProfilePage | null>
  getPageByUserId(userId: UserId): Promise<ProfilePage | null>
  getPageBySlug(slug: string): Promise<ProfilePage | null>

  upsertPage(input: UpsertProfilePageInput): Promise<ProfilePage>
  setVisibility(input: { pageId: ProfilePageId; visibility: ProfilePageVisibility }): Promise<ProfilePage>
  publish(pageId: ProfilePageId): Promise<ProfilePage>
  unpublish(pageId: ProfilePageId): Promise<ProfilePage>

  listBlocks(pageId: ProfilePageId): Promise<ProfileBlock[]>
  upsertBlock(input: UpsertProfileBlockInput): Promise<ProfileBlock>
  deleteBlock(id: ProfileBlockId): Promise<void>
  reorderBlocks(pageId: ProfilePageId, orderedBlockIds: ProfileBlockId[]): Promise<void>
}

