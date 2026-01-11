import type { DomainEvent, EventBus, Unsubscribe } from './EventBus'

/**
 * Event bus simple (in-memory).
 * - Utile pour découpler modules/capabilities dans l'app.
 * - Ne remplace PAS une infra serveur (queues, triggers DB, etc.).
 */
export class InMemoryEventBus implements EventBus {
  private listeners = new Map<string, Set<(event: DomainEvent<any, any>) => void>>()

  publish<TType extends string, TPayload>(event: DomainEvent<TType, TPayload>): void {
    const set = this.listeners.get(event.type)
    if (!set) return
    for (const handler of set) handler(event as DomainEvent<any, any>)
  }

  subscribe<TType extends string, TPayload>(
    type: TType,
    handler: (event: DomainEvent<TType, TPayload>) => void
  ): Unsubscribe {
    let set = this.listeners.get(type)
    if (!set) {
      set = new Set()
      this.listeners.set(type, set)
    }
    set.add(handler as unknown as (event: DomainEvent<any, any>) => void)
    return () => {
      set?.delete(handler as unknown as (event: DomainEvent<any, any>) => void)
      if (set && set.size === 0) this.listeners.delete(type)
    }
  }
}

export const appEventBus = new InMemoryEventBus()

