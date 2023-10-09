import { readable } from "svelte/store";

export type TelegramInitData = {
    query_id: string,
    user: {
        id: string,
        first_name: string,
        last_name: string,
        language_code: string,
        allows_write_to_pm: boolean,
    }
    auth_date: number,
    hash: string,
}

export const parseQuery = (queryString: string): TelegramInitData => {
    const querySplitted = decodeURI(queryString).split('&')
    const result = {};
    // console.log(querySplitted);
    for (let i in querySplitted) {
        // console.log(querySplitted[i])
        let [key, value] = querySplitted[i].split('=')
        if (key === 'user') {
            // @ts-ignore
            result[key] = JSON.parse(decodeURIComponent(value))
        } else {
            // @ts-ignore
            result[key] = value

        }
    }

    return result as TelegramInitData;
}

// @ts-ignore
export const telegramInitData = readable(parseQuery(window.Telegram.WebApp.initData));
