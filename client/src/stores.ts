import { readable } from "svelte/store";

const parseQuery = (queryString: string) => {
    const querySplitted = decodeURI(queryString).split('&')
    const result = {};
    console.log(querySplitted);
    for (let i in querySplitted) {
        console.log(querySplitted[i])
        let [key, value] = querySplitted[i].split('=')
        if (key === 'user') {
            result[key] = value
        } else {
            result[key] = value

        }
    }

    return result;

}

// @ts-ignore
export const telegramInitData = readable(parseQuery(window.Telegram.WebApp.initData));
