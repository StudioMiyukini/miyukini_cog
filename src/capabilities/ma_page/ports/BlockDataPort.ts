export type BlockDataResolverContext = {
  viewerUserId?: string | null
}

export type BlockDataResolver = (input: {
  blockType: string
  dataRef: unknown
  context: BlockDataResolverContext
}) => Promise<unknown>

export type BlockDataPort = {
  registerResolver(blockType: string, resolver: BlockDataResolver): void
  resolve(blockType: string, dataRef: unknown, context: BlockDataResolverContext): Promise<unknown>
}

