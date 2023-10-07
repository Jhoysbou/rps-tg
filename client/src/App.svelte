<script lang="ts">
    import Router from "svelte-spa-router";
    import Home from "./lib/Home.svelte";
    import Matchmaking from "./lib/Matchmaking.svelte";
    import Room from "./lib/Room.svelte";

    const routes = {
        "/": Home,
        "/matchmaking": Matchmaking,
        "/room/:id": Room,
    };
</script>

<main>
    <Router {routes} />
</main>

<style>
</style>
