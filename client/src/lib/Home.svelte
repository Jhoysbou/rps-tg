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
    .header {
        position: relative;
        left: -15vw;
        top: -10vw;
        padding-top: 15vw;
        width: 130vw;
        height: 50vh;
        background: linear-gradient(#5c5470, #b9b4c7);
        border-radius: 0 0 50rem 50rem;
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

    @keyframes cycle-rock {
        from {
        }
        33% {
            margin-left: 2em;
        }
        66% {
            margin-top: 0em;
            margin-left: 1em;
        }
        to {
            margin-top: 2em;
            margin-left: 0em;
        }
    }
</style>
