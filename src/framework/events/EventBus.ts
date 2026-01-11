export type Unsubscribe = () => void

export type DomainEvent<TType extends string = string, TPayload = unknown> = {
  type: TType
  payload: TPayload
  occurredAt: string
}

export interface EventBus {
  publish<TType extends string, TPayload>(
    event: DomainEvent<TType, TPayload>
  ): void

  subscribe<TType extends string, TPayload>(
    type: TType,
    handler: (event: DomainEvent<TType, TPayload>) => void
  ): Unsubscribe
}

