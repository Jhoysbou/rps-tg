<script lang="ts">
    import { push } from "svelte-spa-router";
    import { messenger } from "../main";
    import { telegramInitData, type TelegramInitData } from "../stores";

    let data: Partial<TelegramInitData> = {};
    telegramInitData.subscribe((value) => (data = value));

    const startMatchmaking = () => {
        push("/matchmaking");
        messenger.sendStartMatchmaking();
    };
</script>

<div class="header">
    <div class="name">{`${data.user.first_name} ${data.user.last_name}`}</div>
    <div class="icons">
        <div class="rock">✊</div>
        <div class="paper">✌️</div>
        <div class="scissors">🤚</div>
    </div>
</div>
<button on:click={startMatchmaking}>Start game</button>

<style>
    @keyframes shrink {
        from {
            border-radius: 0;
            height: 100vh;
        }
        to {
        }
    }
    .header {
        position: relative;
        left: -15vw;
        top: -10vw;
        padding-top: 15vw;
        width: 130vw;
        height: 50vh;
        background: linear-gradient(#5c5470, #b9b4c7);
        border-radius: 0 0 50rem 50rem;

        animation-duration: 0.5s;
        animation-name: shrink;
        animation-iteration-count: initial;
        animation-direction: alternate;
        animation-fill-mode: forwards;
    }
    .name {
        font-size: 1.3em;
        font-weight: 600;
    }
    .icons {
        font-size: 200%;
        display: flex;
        justify-content: center;
        align-items: center;
    }
    .rock,
    .scissors {
        padding-top: 2em;
    }
</style>
