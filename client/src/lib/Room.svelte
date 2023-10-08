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

{#if state === RoomState.Choosing}
    <div class=".card">
        <button on:click={() => makeAction(Action.Rock)}>Rock</button>
        <button on:click={() => makeAction(Action.Paper)}>Paper</button>
        <button on:click={() => makeAction(Action.Scissors)}>Scissors</button>
    </div>
{:else if state === RoomState.Waiting}
    <div>Waiting</div>
{:else if state === RoomState.Results}
    <div>Results</div>
{/if}
