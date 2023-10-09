import './app.css'
import App from './App.svelte'
import { Messenger } from './lib/ws/Messenger';
import { WsConnection } from './lib/ws/WsConnection';
import { parseQuery } from './stores';

const generateRandomId = () => {
    return Math.floor(Math.random() * 2 ** 16);
}

let id = parseQuery(window.Telegram.WebApp.initData).user.id

const connection = new WsConnection(
    { url: `ws://bakuta-ad-dev.man.yp-c.yandex.net:8080/ws/${id}` },
);
connection.connect()

export const messenger = new Messenger({ connection });

const app = new App({
    target: document.getElementById('app'),
})

export default app
