import EventEmitter from 'eventemitter3';
import promiseRetry from 'promise-retry';
import { WS_CLOSE_STATUS_NORMAL } from './constants';
import { confirmConnectType } from '../types/messages';

export interface WsConnection {
    once(event: 'connect', listener: () => void): this;
    once(event: 'reconnect', listener: () => void): this;
    once(event: 'disconnect', listener: () => void): this;
    once(event: 'message', listener: (event: MessageEvent) => void): this;

    on(event: 'connect', listener: () => void): this;
    on(event: 'reconnect', listener: () => void): this;
    on(event: 'disconnect', listener: () => void): this;
    on(event: 'message', listener: (event: MessageEvent) => void): this;

    off(event: 'connect', listener: () => void): this;
    off(event: 'reconnect', listener: () => void): this;
    off(event: 'disconnect', listener: () => void): this;
    off(event: 'message', listener: (event: MessageEvent) => void): this;
}

export type WsConnectionConfig = {
    url: string;
};


export class WsConnection extends EventEmitter {
    private ws: WebSocket | null = null;
    private readonly config: WsConnectionConfig;

    constructor(config: WsConnectionConfig) {
        super();

        this.config = config;
    }

    async connect() {
        await this.connectWithRetry();

        this.ws?.addEventListener('close', this.handleClose);
        this.ws?.addEventListener('message', this.handleMessage);
    }

    dispose() {
        if (this.ws) {
            this.removeAllListeners();

            this.ws.removeEventListener('close', this.handleClose);
            this.ws.addEventListener('message', this.handleMessage);
            this.ws.close();
        }
    }

    send(data: string | ArrayBufferLike | Blob | ArrayBufferView) {
        this.ws?.send(data);
    }

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    sendJson(data: any) {
        this.ws?.send(JSON.stringify(data));
    }

    private async connectWithRetry() {
        await promiseRetry((retry) => {
            return this.connectFn().catch(retry);
        });

        this.emit('connect');
    }

    private connectFn() {
        return new Promise<void>((resolve, reject) => {
            this.ws = new WebSocket(this.config.url);

            const messageHandler = (event: MessageEvent) => {
                const message = JSON.parse(event.data);
                const messageEvent = message.type;

                if (messageEvent === confirmConnectType) {
                    this.ws?.removeEventListener('message', messageHandler);
                    message.data.message === 'Connection established' ? resolve() : reject(new Error(message.data.error));
                }
            };
            this.ws.addEventListener('message', messageHandler);

            this.ws.addEventListener(
                'error',
                (event) => {
                    this.ws?.removeEventListener('message', messageHandler);
                    reject(event);
                },
                { once: true },
            );
        });
    }

    private async reconnect() {
        this.emit('reconnect');

        await this.connectWithRetry();
    }

    private handleClose = (event: CloseEvent) => {
        this.emit('disconnect');

        if (event.code !== WS_CLOSE_STATUS_NORMAL) {
            this.reconnect();
        }
    };

    private handleMessage = (event: MessageEvent) => {
        this.emit('message', event);
    };
}
