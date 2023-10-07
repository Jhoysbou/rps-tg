<script lang="ts">
    import { push } from "svelte-spa-router";

    const startMatchmaking = () => {
        push("/matchmaking");
    };
</script>

<button on:click={startMatchmaking}> Start game</button>
