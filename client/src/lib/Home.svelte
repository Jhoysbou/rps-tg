<script lang="ts">
    import { push } from "svelte-spa-router";
    import { messenger } from "../main";
    import { telegramInitData } from "../stores";

    const startMatchmaking = () => {
        push("/matchmaking");
        messenger.sendStartMatchmaking();
    };
</script>

<button on:click={startMatchmaking}> Start game</button>
<div>{JSON.stringify(telegramInitData)}</div>
