<script lang="ts">
    let clockArray = [
        "🕛",
        "🕐",
        "🕑",
        "🕒",
        "🕓",
        "🕔",
        "🕕",
        "🕖",
        "🕗",
        "🕘",
        "🕙",
        "🕚",
    ];

    let timer = 0;

    setInterval(() => {
        timer = timer + 1;
    }, 300);
</script>

<div class="clock">{clockArray[timer % clockArray.length]}</div>

<style>
    .clock {
        font-size: 3em;
    }
</style>
