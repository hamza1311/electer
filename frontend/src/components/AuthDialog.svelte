<script lang="ts">
    import {createEventDispatcher} from "svelte";

    const dispatch = createEventDispatcher();

    let activeIndex = 0;
    const onTabSelect = (e: CustomEvent<{ index: number }>) => {
        activeIndex = e.detail.index
    }
    let username;
    let password;

    const signInOrUp = async (inOrUp: 'in' | 'up') => {
        console.log(username.value, password.value)
        const res = await fetch(`/api/auth/sign${inOrUp}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                username: username.value, password: password.value
            })
        })
        const json = await res.json()
        const payload = {
            exp: new Date(json.duration),
            ...json
        };
        console.log('payload', payload)
        dispatch('signedIn', payload)
    }
    const signIn = () => signInOrUp('in')
    const signUp = () => signInOrUp('up')
</script>

<mwc-tab-bar on:MDCTabBar:activated={onTabSelect}>
    <mwc-tab label="Sign In"></mwc-tab>
    <mwc-tab label="Sign Up"></mwc-tab>
</mwc-tab-bar>

<div class="inputs-container">
    <mwc-textfield label="Username" bind:this={username} />
    <mwc-textfield label="Password" bind:this={password} type="password"/>

    {#if activeIndex === 0}
        <mwc-button on:click={signIn}>Sign In</mwc-button>
    {:else}
        <mwc-button on:click={signUp}>Sign Up</mwc-button>
    {/if}
</div>

<style>
    .inputs-container {
        display: flex;
        flex-direction: column;
        gap: 2rem;
        padding: 2rem;
        align-items: center;
    }

    mwc-textfield, mwc-button {
        width: fit-content;
    }
</style>
