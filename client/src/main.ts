import './app.css'
import App from './App.svelte'
import { Messenger } from './lib/ws/Messenger';
import { WsConnection } from './lib/ws/WsConnection';

const generateRandomId = () => {
    return Math.floor(Math.random() * 2 ** 16);
}

const connection = new WsConnection(
    { url: `ws://bakuta-ad-dev.man.yp-c.yandex.net:8080/ws/${generateRandomId()}` },
);
connection.connect()

export const messenger = new Messenger({ connection });

const app = new App({
    target: document.getElementById('app'),
})

export default app
