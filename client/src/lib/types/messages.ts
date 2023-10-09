/*
* Base type 
* */
type Message<T, D> = {
    type: T,
    data: D,
}


/*
* Confirm connect
* */
export type ConfirmConnectPayload = {
    message: string,
}

export const confirmConnectType = 'ConfirmConnect';
export type ConfirmConnectMessage = Message<typeof confirmConnectType, ConfirmConnectPayload>


/*
* Start matchmaking 
* */
export type StartMatchmakingPayload = null

export const startMatchmakingType = 'StartMatchmaking'
export type StartMatchmakingMessage = Message<typeof startMatchmakingType, StartMatchmakingPayload>


/*
* Matchmaking started
* */
export type MatchmakingStartedPayload = null

export const matchmakingStartedType = 'MatchmakingStarted'
export type MatchmakingStartedMessage = Message<typeof matchmakingStartedType, MatchmakingStartedPayload>


/*
* Matchmaking success
* */
export type MatchmakingSuccessPayload = {
    room: string,
    opponent: string,
}

export const matchmakingSuccessType = 'MatchmakingSuccess'
export type MatchmakingSuccessMessage = Message<typeof matchmakingSuccessType, MatchmakingSuccessPayload>


/*
* Make action
* */
export enum Action {
    Rock = 'Rock',
    Paper = 'Paper',
    Scissors = 'Scissors',
}

export type MakeActionPayload = {
    room: string,
    action: Action,
}

export const makeActionType = 'MakeAction'
export type MakeActionMessage = Message<typeof makeActionType, MakeActionPayload>


/*
* Make action success
* */
export type MakeActionSuccessPayload = null;

export const makeActionSuccessType = 'MakeActionSuccess'
export type MakeActionSuccessMessage = Message<typeof makeActionSuccessType, MakeActionPayload>


/*
* Round finished
* */
export type ActionHistory = {
    user_id: number,
    action: Action,
}

export type RoundFinishedPayload = {
    winner: number,
    actions: ActionHistory[],
    next_round_count: number,
};

export const roundFinishedType = 'RoundFinished'
export type RoundFinishedMessage = Message<typeof roundFinishedType, RoundFinishedPayload>


/*
* Game finished
* */
export type GameFinishedPayload = {
    winner: number,
    actions: ActionHistory[],
};

export const gameFinishedType = 'GameFinished'
export type GameFinishedMessage = Message<typeof gameFinishedType, GameFinishedPayload>
