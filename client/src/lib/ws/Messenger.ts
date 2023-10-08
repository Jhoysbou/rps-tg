import EventEmitter from "eventemitter3";
import type { WsConnection } from "./WsConnection";
import { startMatchmakingType, matchmakingStartedType, type MatchmakingStartedPayload, matchmakingSuccessType, type MatchmakingSuccessPayload, type MakeActionPayload, makeActionType, makeActionSuccessType, type MakeActionSuccessPayload, roundFinishedType, type RoundFinishedPayload, gameFinishedType, type GameFinishedPayload } from "../types/messages";

export type MessengerConfig = {
    connection: WsConnection;
};

type IncomingMessages = {
    [matchmakingStartedType]: MatchmakingStartedPayload;
    [matchmakingSuccessType]: MatchmakingSuccessPayload;
    [makeActionSuccessType]: MakeActionSuccessPayload;
    [roundFinishedType]: RoundFinishedPayload;
    [gameFinishedType]: GameFinishedPayload;
};

type IncomingMessageType = keyof IncomingMessages

export class Messenger extends EventEmitter {
    private readonly connection: WsConnection;

    constructor({ connection }: MessengerConfig) {
        super();
        this.connection = connection;
        this.connection.on('message', this.handleMessage);
    }

    sendStartMatchmaking() {
        this.connection.sendJson({
            type: startMatchmakingType,
        })
    }

    sendMakeAction(data: MakeActionPayload) {
        this.connection.sendJson({
            type: makeActionType,
            data,
        })
    }

    dispose() {
        this.connection.off('message', this.handleMessage);
    }

    // @ts-ignore
    on<TType extends IncomingMessageType>(type: TType, listener: (payload: IncomingMessages[TType]) => void): this {
        return super.on(type, listener);
    }

    // @ts-ignore
    once<TType extends IncomingMessageType>(type: TType, listener: (payload: IncomingMessages[TType]) => void): this {
        return super.once(type, listener);
    }

    // @ts-ignore
    off<TType extends IncomingMessageType>(type: TType, listener: (payload: IncomingMessages[TType]) => void): this {
        return super.off(type, listener);
    }

    // @ts-ignore
    emit<TType extends IncomingMessageType>(type: TType, payload: IncomingMessages[TType]): boolean {
        return super.emit(type, payload);
    }

    private handleMessage = (event: MessageEvent) => {
        const message = JSON.parse(event.data);
        const messageEvent = message.type;

        switch (messageEvent) {
            case matchmakingStartedType:
                this.emit(matchmakingStartedType, message.data);
                break;
            case matchmakingSuccessType:
                this.emit(matchmakingSuccessType, message.data);
                break;
            case makeActionSuccessType:
                this.emit(makeActionSuccessType, message.data);
                break;
            case roundFinishedType:
                this.emit(roundFinishedType, message.data);
                break;
            case gameFinishedType:
                this.emit(gameFinishedType, message.data);
                break;
        }
    };
}
