<script lang="ts">
    import { replace } from "svelte-spa-router";
    import { messenger } from "../main";
    import { telegramInitData, type TelegramInitData } from "../stores";
    import { RoomState } from "./constants";
    import Loader from "./room/Loader.svelte";
    import { Action, type ActionHistory } from "./types/messages";
    import { fade } from "svelte/transition";

    export let params;
    let { id, ...other } = params;

    let data: Partial<TelegramInitData> = {};
    telegramInitData.subscribe((value) => (data = value));

    let state: RoomState = RoomState.Choosing;
    let isWinner = false;
    let actionHistory: Action[] = [];
    let opponentActionHistory: Action[] = [];
    let wins = 0;
    let opponentWins = 0;

    const updateOpponentHistory = (actions: ActionHistory[]) => {
        const opponentAction = actions.filter(
            (a) => a.user_id !== data.user.id
        )[0];
        opponentActionHistory = [
            ...opponentActionHistory,
            opponentAction.action,
        ];
    };

    const updateWins = (winner: number) => {
        if (winner === null) {
            return;
        }

        if (winner === data.user.id) {
            wins = wins + 1;
        } else {
            opponentWins = opponentWins + 1;
        }
    };

    messenger.on("RoundFinished", (payload) => {
        updateOpponentHistory(payload.actions);
        updateWins(payload.winner);
        state = RoomState.Choosing;
    });
    messenger.on("GameFinished", (payload) => {
        updateOpponentHistory(payload.actions);
        updateWins(payload.winner);
        if (payload.winner === data.user.id) {
            isWinner = true;
        }
        state = RoomState.Results;
    });

    const makeAction = (action: Action) => {
        state = RoomState.Waiting;
        actionHistory = [...actionHistory, action];
        messenger.sendMakeAction({ action, room: id });
    };

    let actionMap = {
        [Action.Rock]: "✊",
        [Action.Paper]: "🤚",
        [Action.Scissors]: "✌️",
    };

    const goHome = () => {
        replace("/");
    };
</script>

<div class={"background" + `${state === RoomState.Results ? " expand" : ""}`}>
    <div class="opponent_info">
        <p>Jonh Doe</p>
        <div>{opponentWins}</div>
        <div class="history">
            {#each opponentActionHistory as action}
                <div class="history_item">{actionMap[action]}</div>
            {/each}
        </div>
    </div>
    {#if state === RoomState.Results}
        <div class="info">
            <h1>{isWinner ? "You won!" : "You lost!"}</h1>
            <div>{wins}</div>
            <div class="history">
                {#each actionHistory as action}
                    <div class="history_item">{actionMap[action]}</div>
                {/each}
            </div>
        </div>
        <button
            class="home"
            on:click={goHome}
            in:fade={{ delay: 500, duration: 200 }}
        >
            Home
        </button>
    {/if}
</div>
<div>
    {#if state === RoomState.Choosing}
        <div class="card">
            <button on:click={() => makeAction(Action.Rock)}>✊</button>
            <button on:click={() => makeAction(Action.Paper)}>🤚</button>
            <button on:click={() => makeAction(Action.Scissors)}>✌️</button>
        </div>
    {:else if state === RoomState.Waiting}
        <Loader />
    {/if}
    {#if state !== RoomState.Results}
        <div>{wins}</div>
        <div class="history">
            {#each actionHistory as action}
                <div class="history_item">{actionMap[action]}</div>
            {/each}
        </div>
    {/if}
</div>

<style>
    @keyframes shrink {
        from {
        }
        to {
            top: 0vw;
            height: 40vh;
            border-radius: 0 0 50em 50em;
        }
    }
    @keyframes expand {
        from {
            top: -40vw;
            height: 50vh;
            border-radius: 0 0 50em 50em;
        }
        to {
            top: 0vh;
            padding-top: 0;
            height: 100vh;
            border-radius: 0;
        }
    }
    .background {
        align-items: center;
        position: relative;
        left: -15vw;
        top: -10vw;
        padding-top: 15vw;
        width: 130vw;
        height: 115vh;
        background: linear-gradient(#5c5470, #b9b4c7);
        border-radius: 0;
        display: flex;
        flex-direction: column;

        background: linear-gradient(#5c5470, #b9b4c7);

        animation-duration: 0.5s;
        animation-name: shrink;
        animation-iteration-count: initial;
        animation-direction: alternate;
        animation-fill-mode: forwards;
    }

    .info {
        margin: 5vh;
    }

    .opponent_info > p {
        font-size: 1.3em;
        font-weight: 500;
    }

    .history {
        display: flex;
        justify-content: center;
    }
    .history > div {
        border-radius: 5px;
        margin: 3px;
        padding: 2px;
        background-color: #5c5470;
    }

    .expand {
        animation-duration: 0.5s;
        animation-name: expand;
        animation-iteration-count: initial;
        animation-direction: alternate;
        animation-fill-mode: forwards;
    }
</style>
