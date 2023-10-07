<script lang="ts">
    import { replace } from "svelte-spa-router";

    let count: number = 0;
    const increment = () => {
        count += 1;
    };
    setInterval(increment, 1000);
    setTimeout(() => replace("/room/37461"), 3000);
</script>

<h1>
    {count}
</h1>
