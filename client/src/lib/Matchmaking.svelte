<script lang="ts">
    import { replace } from "svelte-spa-router";
    import { messenger } from "../main";

    messenger.once("MatchmakingSuccess", (payload) => {
        replace(`/room/${payload.room}`);
    });
    let count: number = 0;
    const increment = () => {
        count += 1;
    };
    setInterval(increment, 1000);
</script>

<h1>
    {count}
</h1>
