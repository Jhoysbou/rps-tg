<script lang="ts">
    import { replace } from "svelte-spa-router";
    import { messenger } from "../main";
    import { fade } from "svelte/transition";

    messenger.once("MatchmakingSuccess", (payload) => {
        setTimeout(() => replace(`/room/${payload.room}`), 1000);
    });
    let count: number = 0;
    const increment = () => {
        count += 1;
    };
    setInterval(increment, 1000);
</script>

<div class="background">
    <h1 class="counter">
        {count}
    </h1>
    <button class="cancel" in:fade={{ delay: 500, duration: 200 }}>
        Cancel
    </button>
</div>

<style>
    @keyframes expand {
        from {
        }
        to {
            height: 115vh;
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
        height: 50vh;
        background: linear-gradient(#5c5470, #b9b4c7);
        border-radius: 0 0 50rem 50rem;

        background: linear-gradient(#5c5470, #b9b4c7);

        animation-duration: 0.5s;
        animation-name: expand;
        animation-iteration-count: initial;
        animation-direction: alternate;
        animation-fill-mode: forwards;
    }
    .cancel {
        margin-top: 35vh;
    }
</style>
