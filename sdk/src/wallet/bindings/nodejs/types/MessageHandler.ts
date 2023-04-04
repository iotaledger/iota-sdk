import type {
    EventType,
    __Message__,
    __AccountMethod__,
    AccountId,
} from './';

export interface MessageHandler {
    messageHandler: object,
    sendMessage(message: __Message__): Promise<string>,
    callAccountMethod(accountIndex: AccountId, method: __AccountMethod__): Promise<string>,
    listen(eventTypes: EventType[], callback: (error: Error, result: string) => void): Promise<void>,
    destroy(): Promise<void>

}