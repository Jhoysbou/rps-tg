<script lang="ts">
    import { messenger } from "../main";
    import { RoomState } from "./constants";
    import { Action } from "./types/messages";

    export let params;
    let { id, ...other } = params;

    let state: RoomState = RoomState.Choosing;

    messenger.on("RoundFinished", (payload) => {
        console.log(payload);
        state = RoomState.Choosing;
    });
    messenger.on("GameFinished", (payload) => {
        console.log(payload);
        state = RoomState.Results;
    });

    const makeAction = (action: Action) => {
        state = RoomState.Waiting;
        messenger.sendMakeAction({ action, room: id });
    };
</script>

<div class="background">
    <div class="opponent_info">
        <p>Jonh Doe</p>
        some text
    </div>
</div>
<div>
    {#if state === RoomState.Choosing}
        <div class="card">
            <button on:click={() => makeAction(Action.Rock)}>✊</button>
            <button on:click={() => makeAction(Action.Paper)}>✌️</button>
            <button on:click={() => makeAction(Action.Scissors)}>🤚</button>
        </div>
    {:else if state === RoomState.Waiting}
        <div>Waiting</div>
    {:else if state === RoomState.Results}
        <div>Results</div>
    {/if}
</div>

<style>
    @keyframes shrink {
        from {
        }
        to {
            top: -40vw;
            height: 50vh;
            border-radius: 0 0 50em 50em;
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

        background: linear-gradient(#5c5470, #b9b4c7);

        animation-duration: 0.5s;
        animation-name: shrink;
        animation-iteration-count: initial;
        animation-direction: alternate;
        animation-fill-mode: forwards;
    }

    .opponent_info > p {
        font-size: 1.3em;
        font-weight: 500;
        margin-top: 25vh;
    }
</style>
