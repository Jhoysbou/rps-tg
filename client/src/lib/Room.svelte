<script lang="ts">
    import { RoomState } from "./constants";

    export let params;
    let { id, ...other } = params;

    let state: RoomState = RoomState.Choosing;

    console.log(id);

    const makeAction = (action: string) => {
        state = RoomState.Waiting;
    };
</script>

{#if state === RoomState.Choosing}
    <div class=".card">
        <button on:click={() => makeAction("Rock")}>Rock</button>
        <button on:click={() => makeAction("Paper")}>Paper</button>
        <button on:click={() => makeAction("Scissors")}>Scissors</button>
    </div>
{:else if state === RoomState.Waiting}
    <div>Waiting</div>
{:else if state === RoomState.Results}
    <div>Results</div>
{/if}
